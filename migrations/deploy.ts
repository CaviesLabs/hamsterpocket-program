// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
import { create } from "lodash";

const path = require("path");
require("dotenv").config({ path: path.join(__dirname, "../.env") });

import * as anchor from "@project-serum/anchor";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { Market, OpenOrders } from "@openbook-dex/openbook";
import { AnchorProvider } from "@project-serum/anchor/dist/cjs/provider";
import { BorshCoder, EventParser } from "@project-serum/anchor";

import { getFixtures } from "../tests/test.helper";
import {
  closeAccount,
  createWrappedNativeAccount, getAssociatedTokenAddress,
} from "@solana/spl-token";

type Fixtures = Awaited<ReturnType<typeof getFixtures>>;


// mango market
const mangoMarket = {
  "programId": "DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY",
  "baseMint": "So11111111111111111111111111111111111111112",
  "quoteMint": "8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN",
  "name": "SOL/USDC",
  "publicKey": "5xWpt56U1NCuHoAEtpLeUrQcxDkEpNfScjfLFaRzLPgR",
  "baseSymbol": "SOL",
  "baseDecimals": 9,
  "quoteDecimals": 6,
  "marketIndex": 3,
  "bidsKey": "8ezpneRznTJNZWFSLeQvtPCagpsUVWA7djLSzqp3Hx4p",
  "asksKey": "8gJhxSwbLJkDQbqgzbJ6mDvJYnEVWB6NHWEN9oZZkwz7",
  "eventsKey": "48be6VKEq86awgUjfvbKDmEzXr4WNR7hzDxfF6ZPptmd"
};

/**
 * @dev Execute swap
 * @param provider
 * @param fixtures
 */
const executeSwap = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    pocketAccount,
    baseMintVaultAccount,
    targetMintVaultAccount,
    pocketRegistry,
    program,
    deployer
  } = fixtures;

  const operator = Keypair.fromSecretKey(
    anchor.utils.bytes.bs58.decode(
      process.env.OPERATOR_KEY
    )
  );

  let marketAddress = new PublicKey(mangoMarket.publicKey);
  let programAddress = new PublicKey(mangoMarket.programId);

  let market = await Market.load(provider.connection, marketAddress, {}, programAddress);

  const desiredOpenOrderAccount = Keypair.generate();

  const openOrders = await market.loadOrdersForOwner(provider.connection, pocketAccount);
  let initInx = [];

  if (openOrders.length === 0) {
    initInx.push(
      await program.methods.initializePocketDexRegistry().accounts({
        signer: operator.publicKey,
        pocket: pocketAccount,
        pocketRegistry,
        market: market.publicKey,
        openOrders: desiredOpenOrderAccount.publicKey,
        dexProgram: new PublicKey(mangoMarket.programId)
      }).instruction()
    )
  }

  const txId = await program.methods.executeSwap().accounts({
    // pocket accounts
    signer: operator.publicKey,
    pocket: pocketAccount,
    pocketRegistry,
    pocketBaseTokenVault: baseMintVaultAccount,
    pocketTargetTokenVault: targetMintVaultAccount,
    // serum dex accounts
    market: market.publicKey,
    eventQueue: market.decoded.eventQueue,
    requestQueue: market.decoded.requestQueue,
    coinVault: market.decoded.baseVault,
    pcVault: market.decoded.quoteVault,
    marketBids: market.decoded.bids,
    marketAsks: market.decoded.asks,
    openOrders: desiredOpenOrderAccount.publicKey,
    dexProgram: new PublicKey(mangoMarket.programId)
  }).preInstructions(initInx).signers([deployer.payer]).rpc({ commitment: "confirmed" }).catch(e => console.log(e));

  // expect log
  const transaction = await provider.connection.getParsedTransaction(txId as string, {
    commitment: "confirmed"
  });

  const eventParser = new EventParser(
    program.programId,
    new BorshCoder(program.idl)
  );

  const [event] = eventParser.parseLogs(transaction.meta.logMessages);

  console.log({ event });
};

const initializeAccount = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    pocketRegistry,
    program,
    deployer
  } = fixtures;

  const operator = Keypair.fromSecretKey(
    anchor.utils.bytes.bs58.decode(
      process.env.OPERATOR_KEY
    )
  );

  await program.methods.initialize({
    operators: [operator.publicKey]
  }).accounts({
    pocketRegistry,
    owner: deployer.publicKey
  }).signers([deployer.payer])
    .rpc({ commitment: "confirmed" })
    .catch(e => console.log(e));
};

const createPocket = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    pocketId,
    program,
    pocketRegistry,
    pocketAccount,
    baseMintAccount,
    targetMintAccount,
    baseMintVaultAccount,
    targetMintVaultAccount,
    deployer
  } = fixtures;

  const operator = Keypair.fromSecretKey(
    anchor.utils.bytes.bs58.decode(
      process.env.OPERATOR_KEY
    )
  );

  await program.methods.updatePocketRegistry({
    operators: [operator.publicKey]
  }).accounts({
    pocketRegistry,
    owner: deployer.publicKey
  }).signers([deployer.payer])
    .rpc({ commitment: "confirmed" })
    .catch(e => console.log(e));

  await closeAccount(
    provider.connection,
    deployer.payer,
    await getAssociatedTokenAddress(baseMintAccount, deployer.publicKey),
    deployer.publicKey,
    deployer.publicKey
  );

  const ownerBaseTokenAccount = await createWrappedNativeAccount(
    provider.connection,
    deployer.payer,
    deployer.publicKey,
    LAMPORTS_PER_SOL * 1
  );

  const inx = [
    await program.methods
      .createTokenVault()
      .accounts({
        mintAccount: baseMintAccount,
        pocketTokenVault: baseMintVaultAccount,
        signer: deployer.publicKey,
        pocket: pocketAccount
      })
      .instruction(),

    await program.methods
      .createTokenVault()
      .accounts({
        mintAccount: targetMintAccount,
        pocketTokenVault: targetMintVaultAccount,
        signer: deployer.publicKey,
        pocket: pocketAccount
      })
      .instruction(),

    await program.methods
      .deposit({
        depositAmount: new anchor.BN(LAMPORTS_PER_SOL * 1)
      })
      .accounts({
        signer: deployer.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        signerTokenAccount: ownerBaseTokenAccount
      }).instruction()
  ];

  const pocketData = {
    id: pocketId,
    side: { buy: {} },
    targetTokenAddress: baseMintAccount,
    baseTokenAddress: targetMintAccount,
    stopConditions: [],
    buyCondition: null,
    startAt: new anchor.BN(new Date().getTime().toString()),
    batchVolume: new anchor.BN((LAMPORTS_PER_SOL * 0.01).toString()),
    name: "pocket name",
    frequency: { hours: new anchor.BN(1) }
  };

  await program.methods
    // @ts-ignore
    .createPocket(pocketData)
    .accounts({
      pocket: pocketAccount,
      signer: deployer.publicKey,
      pocketRegistry
    })
    .postInstructions(inx)
    .signers([deployer.payer])
    .rpc({ commitment: "confirmed" })
    .catch((e) => console.log(e));

  const pocket = await program.account.pocket.fetch(pocketAccount);
  console.log({ pocket });
};

module.exports = async function(provider: AnchorProvider) {
  const fixtures = await getFixtures(provider, {
    baseMint: new PublicKey("So11111111111111111111111111111111111111112"),
    quoteMint: new PublicKey("8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN"),
    pocketId: "BuyUWxbiQS7fGU7kqvSanMtq"
  });

  // await initializeAccount(provider, fixtures);
  // await createPocket(provider, fixtures);
  await executeSwap(provider, fixtures);
};
