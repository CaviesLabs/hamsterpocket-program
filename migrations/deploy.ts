// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
const path = require("path");
require("dotenv").config({ path: path.join(__dirname, "../.env") });

import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { Market, OpenOrders } from "@openbook-dex/openbook";

import {
  closeAccount,
  createAssociatedTokenAccountInstruction,
  createCloseAccountInstruction,
  createSyncNativeInstruction,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  NATIVE_MINT
} from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor/dist/cjs/provider";
import { BorshCoder, EventParser, IdlTypes } from "@project-serum/anchor";

import { IDL, Pocket } from "../target/types/pocket";

type ProgramPocketTypes = IdlTypes<Pocket>;
type Fixtures = Awaited<ReturnType<typeof getFixtures>>;

// https://api.raydium.io/v2/sdk/liquidity/mainnet.json
// Raydium market
const marketSOLUSDT = {
  "id": "7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX",
  "baseMint": "So11111111111111111111111111111111111111112",
  "quoteMint": "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
  "lpMint": "Epm4KfTj4DMrvqn6Bwg2Tr2N8vhQuNbuK8bESFp4k33K",
  "baseDecimals": 9,
  "quoteDecimals": 6,
  "lpDecimals": 9,
  "version": 4,
  "programId": "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
  "authority": "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
  "openOrders": "3oWQRLewGsUMA2pebcpGPPGrzyRNfbs7fQEMUxPAGgff",
  "targetOrders": "9x4knb3nuNAzxsV7YFuGLgnYqKArGemY54r2vFExM1dp",
  "baseVault": "876Z9waBygfzUrwwKFfnRcc7cfY4EQf6Kz1w7GRgbVYW",
  "quoteVault": "CB86HtaqpXbNWbq67L18y5x2RhqoJ6smb7xHUcyWdQAQ",
  "withdrawQueue": "52AfgxYPTGruUA9XyE8eF46hdR6gMQiA6ShVoMMsC6jQ",
  "lpVault": "2JKZRQc92TaH3fgTcUZyxfD7k7V7BMqhF24eussPtkwh",
  "marketVersion": 4,
  "marketProgramId": "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
  "marketId": "2AdaV97p6SfkuMQJdu8DHhBhmJe7oWdvbm52MJfYQmfA",
  "marketAuthority": "n8meSpYX5n3oRoToN21PFQ5SSYBDf675eub3WMoJJoA",
  "marketBaseVault": "4zVFCGJVQhSvsJ625qTH4WKgvfPQpNpAVUfjpgCxbKh8",
  "marketQuoteVault": "9aoqhYjXBqWsTVCEjwtxrotx6sVPGVLmbpVSpSRzTv54",
  "marketBids": "F4LnU7SarP7nLmGPnDHxnCqZ8gRwiFRgbo5seifyicfo",
  "marketAsks": "BKgZNz8tqJFoZ9gEHKR6k33wBMeXKAaSWpW5zMhSRhr3",
  "marketEventQueue": "9zw6ztEpHfcKccahzTKgPkQNYhJMPwL4iJJc8BAztNYY",
  "lookupTableAccount": "4vuTWb2bevuagtCg6ap4pNMRGTVbZ95zAtSJvJtKJdfv"
};


const getFixtures = async (provider: AnchorProvider, opt?: {
  pocketId?: string
}) => {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  provider.opts.commitment = "confirmed";

  const program = new anchor.Program(IDL, process.env.PROGRAM_ID);
  const deployer = provider.wallet as anchor.Wallet;

  const pocketId = opt?.pocketId || Keypair.generate().publicKey.toString().slice(0, 24);

  // find the pocket account
  const [pocketRegistry] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("SEED::POCKET::PLATFORM")],
    program.programId
  );

  // find the pocket account
  const [pocketAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::POCKET_SEED"),
      anchor.utils.bytes.utf8.encode(pocketId)
    ],
    program.programId
  );

  const baseMintAccount = new PublicKey(marketSOLUSDT.baseMint);
  const [baseMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      baseMintAccount.toBytes()
    ],
    program.programId
  );

  const targetMintAccount = new PublicKey(marketSOLUSDT.quoteMint);
  const [targetMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      targetMintAccount.toBytes()
    ],
    program.programId
  );

  return {
    pocketId,
    program,
    pocketRegistry,
    pocketAccount,
    baseMintAccount,
    targetMintAccount,
    baseMintVaultAccount,
    targetMintVaultAccount,
    deployer
  };
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

  const operator = deployer.publicKey;

  let marketAddress = new PublicKey(marketSOLUSDT.marketId);
  let programAddress = new PublicKey(marketSOLUSDT.marketProgramId);

  const pocket = await program.account.pocket.fetch(pocketAccount);
  console.log(new Date(pocket.startAt.toNumber() * 1000));

  let market = await Market.load(provider.connection, marketAddress, {}, programAddress);

  // const desiredOpenOrderAccount = new Keypair();
  const desiredOpenOrderAccount = await PublicKey.createWithSeed(
    deployer.publicKey,
    pocketAccount.toString().slice(0, 32),
    programAddress
  );

  const initInx = [];

  if (!await provider.connection.getAccountInfo(desiredOpenOrderAccount)) {
    initInx.push(
      SystemProgram.createAccountWithSeed({
        basePubkey: deployer.publicKey,
        fromPubkey: deployer.publicKey,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(
          OpenOrders.getLayout(programAddress).span
        ),
        newAccountPubkey: desiredOpenOrderAccount,
        programId: programAddress,
        seed: pocketAccount.toString().slice(0, 32),
        space: OpenOrders.getLayout(programAddress).span
      })
    );

    initInx.push(
      await program.methods.initSwapRegistry().accounts({
        marketKey: marketAddress,
        authority: pocketAccount,
        openOrders: desiredOpenOrderAccount,
        dexProgram: programAddress,
        pocket: pocketAccount
      }).instruction()
    );
  }
  ;

  const cleanUpInx = [];

  cleanUpInx.push(
    await program.methods.closeSwapRegistry().accounts({
      marketKey: marketAddress,
      authority: pocketAccount,
      destination: deployer.publicKey,
      openOrders: desiredOpenOrderAccount,
      dexProgram: programAddress,
      pocket: pocketAccount
    }).instruction()
  );

  const txId = await program.methods.executeSwap().accounts({
    // pocket accounts
    marketKey: market.publicKey,
    signer: operator,
    pocket: pocketAccount,
    pocketRegistry,
    pocketBaseTokenVault: baseMintVaultAccount,
    pocketQuoteTokenVault: targetMintVaultAccount
  })
    .preInstructions(initInx)
    .remainingAccounts([
      // serum dex accounts
      { pubkey: market.decoded.eventQueue, isSigner: false, isWritable: true },
      { pubkey: market.decoded.requestQueue, isSigner: false, isWritable: true },
      { pubkey: market.decoded.bids, isSigner: false, isWritable: true },
      { pubkey: market.decoded.asks, isSigner: false, isWritable: true },
      { pubkey: market.decoded.baseVault, isSigner: false, isWritable: true },
      { pubkey: market.decoded.quoteVault, isSigner: false, isWritable: true },
      { pubkey: new PublicKey(marketSOLUSDT.marketAuthority), isSigner: false, isWritable: false },
      { pubkey: desiredOpenOrderAccount, isSigner: false, isWritable: true },
      { pubkey: programAddress, isSigner: false, isWritable: false }
    ]).signers([deployer.payer])
    // .rpc({ commitment: "confirmed" })
    .simulate({ commitment: "confirmed" })
    .catch(e => console.log(e));

  // // expect log
  const transaction = await provider.connection.getParsedTransaction(txId as any, {
    commitment: "confirmed"
  });

  const eventParser = new EventParser(
    program.programId,
    new BorshCoder(program.idl)
  );

  const events = eventParser.parseLogs(transaction.meta.logMessages);

  for (let event of events) {
    console.log(event.name, event.data);
  }
};

const initializeAccount = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    pocketRegistry,
    program,
    deployer
  } = fixtures;

  await program.methods.initialize({
    operators: [deployer.publicKey]
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

  const ownerBaseTokenAccount = await getAssociatedTokenAddress(
    NATIVE_MINT,
    deployer.publicKey
  );

  const ownerTargetTokenAccount = await getAssociatedTokenAddress(
    targetMintAccount,
    deployer.publicKey
  );

  await closeAccount(
    provider.connection,
    deployer.payer,
    ownerBaseTokenAccount,
    deployer.publicKey,
    deployer.publicKey
  ).catch(e => console.log(e));

  const inx = [
    createAssociatedTokenAccountInstruction(
      deployer.publicKey,
      ownerBaseTokenAccount,
      deployer.publicKey,
      NATIVE_MINT
    ),
    SystemProgram.transfer({
      fromPubkey: deployer.publicKey,
      toPubkey: ownerBaseTokenAccount,
      lamports: LAMPORTS_PER_SOL * 0.5
    }),
    createSyncNativeInstruction(
      ownerBaseTokenAccount
    ),
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
        depositAmount: new anchor.BN(LAMPORTS_PER_SOL * 0.5),
        mode: { base: {} }
      })
      .accounts({
        signer: deployer.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        pocketQuoteTokenVault: targetMintVaultAccount,
        signerBaseTokenAccount: ownerBaseTokenAccount,
        signerQuoteTokenAccount: ownerTargetTokenAccount
      }).instruction()
  ];

  const pocketData: ProgramPocketTypes["CreatePocketParams"] = {
    id: pocketId,
    side: { sell: {} },
    baseTokenAddress: baseMintAccount,
    quoteTokenAddress: targetMintAccount,
    marketKey: new PublicKey(marketSOLUSDT.marketId),
    stopConditions: [{
      batchAmountReach: {
        value: new anchor.BN("1"),
        // @ts-ignore
        isPrimary: true
      }
    }],
    buyCondition: {
      gte: {
        value: new anchor.BN("1000000")
      }
    },
    startAt: new anchor.BN(parseInt(String(new Date().getTime() / 1000 + 10))),
    batchVolume: new anchor.BN((LAMPORTS_PER_SOL * 0.1).toString()),
    name: "pocket name",
    frequency: { hours: new anchor.BN(1) }
  };

  await program.methods
    .createPocket(pocketData)
    .accounts({
      pocket: pocketAccount,
      signer: deployer.publicKey
    })
    .postInstructions(inx)
    .signers([deployer.payer])
    .rpc({ commitment: "confirmed" })
    .catch((e) => console.log(e));

  const pocket = await program.account.pocket.fetch(pocketAccount);
  console.log({ pocket });
};

const addOperator = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    program,
    pocketRegistry,
    deployer
  } = fixtures;

  // const operator = new PublicKey("HdWkKSDM2UDdtbiwh2fAHunyDckEnZde6mehYMH2hiBq");
  const operator = new PublicKey("FDe1Kp6FyrmJKMdnG1yuxFyGynq8wgrHTaz1UFNQ5Y5E");

  await program.methods.updatePocketRegistry({
    operators: [operator, deployer.publicKey]
  }).accounts({
    pocketRegistry,
    owner: deployer.publicKey
  }).signers([deployer.payer])
    .rpc({ commitment: "processed" })
    .catch(e => console.log(e));
};


const cancelAndWithdraw = async (provider: AnchorProvider, fixtures: Fixtures) => {
  const {
    program,
    pocketAccount,
    baseMintAccount,
    targetMintAccount,
    baseMintVaultAccount,
    targetMintVaultAccount,
    deployer
  } = fixtures;

  const inx = [];

  const ownerBaseTokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    baseMintAccount,
    deployer.publicKey
  );
  const ownerTargetTokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    deployer.publicKey
  );

  inx.push(await program.methods.updatePocket({
    status: { closed: {} }
  }).accounts({
    pocket: pocketAccount,
    signer: deployer.publicKey
  }).instruction());


  inx.push(await program.methods
    .withdraw()
    .accounts({
      signer: deployer.publicKey,
      pocket: pocketAccount,
      pocketBaseTokenVault: baseMintVaultAccount,
      pocketQuoteTokenVault: targetMintVaultAccount,
      signerBaseTokenAccount: ownerBaseTokenAccount.address,
      signerQuoteTokenAccount: ownerTargetTokenAccount.address
    })
    .instruction());

  inx.push(await createCloseAccountInstruction(
    ownerBaseTokenAccount.address,
    deployer.publicKey,
    deployer.publicKey
  ));

  await provider.sendAndConfirm(
    new Transaction().add(...inx),
    [deployer.payer]
  ).catch(e => console.log(e));
};

module.exports = async function(provider: AnchorProvider) {
  const fixtures = await getFixtures(provider, {
    pocketId: null
  });
  // const pocket = await fixtures.program.account.pocket.fetch(fixtures.pocketAccount);
  // console.log({ pocket });
  // console.log(pocket.frequency.hours.toNumber());
  // console.log(pocket.stopConditions.map(elm => console.log(
  //   new Date(elm.endTimeReach.value.toNumber() * 1000)
  // )));
  // await initializeAccount(provider, fixtures);
  await addOperator(provider, fixtures);
  // await createPocket(provider, fixtures);
  // await executeSwap(provider, fixtures);
  // await cancelAndWithdraw(provider, fixtures);
};
