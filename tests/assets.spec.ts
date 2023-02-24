import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";
import { BorshCoder, EventParser } from "@project-serum/anchor";

import { getFixtures } from "./test.helper";

describe("assets", async () => {
  let fixtures: Awaited<ReturnType<typeof getFixtures>>;

  before(async () => {
    fixtures = await getFixtures(anchor.AnchorProvider.env());

    /**
     * @dev Create a pocket
     */
    const {
      pocketId,
      program,
      pocketRegistry,
      targetMintAccount,
      baseMintAccount,
      pocketAccount,
      owner,
      baseMintVaultAccount,
      targetMintVaultAccount,
    } = fixtures;

    const pocketData = {
      id: pocketId,
      side: {buy: {}},
      targetTokenAddress: targetMintAccount,
      baseTokenAddress: baseMintAccount,
      stopConditions: [],
      buyCondition: null,
      startAt: new anchor.BN(new Date().getTime().toString()),
      batchVolume: new anchor.BN((LAMPORTS_PER_SOL * 10).toString()),
      name: "pocket name",
      frequency: { hours: new anchor.BN(1) },
    };

    const inx = [
      await program.methods
        .createTokenVault()
        .accounts({
          mintAccount: baseMintAccount,
          pocketTokenVault: baseMintVaultAccount,
          signer: owner.publicKey,
          pocket: pocketAccount,
        })
        .instruction(),
      await program.methods
        .createTokenVault()
        .accounts({
          mintAccount: targetMintAccount,
          pocketTokenVault: targetMintVaultAccount,
          signer: owner.publicKey,
          pocket: pocketAccount,
        })
        .instruction(),
    ];

    await program.methods
      // @ts-ignore
      .createPocket(pocketData)
      .accounts({
        pocket: pocketAccount,
        signer: owner.publicKey,
        pocketRegistry,
      })
      .signers([owner])
      .postInstructions(inx)
      .rpc({ commitment: "confirmed" })
      .then(r => r)
      .catch((e) => console.log(e));
  });

  it("[deposit] should: owner can deposit assets to pocket successfully", async () => {
    const {
      provider,
      program,
      pocketAccount,
      owner,
      baseMintAccount,
      baseMintVaultAccount,
      ownerBaseTokenAccount,
    } = fixtures;

    const txId = await program.methods
      .deposit({
        depositAmount: new anchor.BN(LAMPORTS_PER_SOL * 2),
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        signerTokenAccount: ownerBaseTokenAccount,
      })
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.baseTokenBalance.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;

    // expect log
    const transaction = await provider.connection.getParsedTransaction(txId as string, {
      commitment: "confirmed",
    });
    const eventParser = new EventParser(
      program.programId,
      new BorshCoder(program.idl)
    );
    const [event] = eventParser.parseLogs(transaction.meta.logMessages);

    // Expect emitted logs
    expect(event.name).eq('PocketDeposited');
    expect(
      (event.data as any).owner.equals(owner.publicKey)
    ).equals(true);
    expect(
      (event.data as any).pocketAddress.equals(pocketAccount)
    ).equals(true);
     expect(
      (event.data as any).mintAddress.equals(baseMintAccount)
    ).equals(true);
     expect(
      (event.data as any).amount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))
    ).equals(true);

  });

  it("[withdraw] should: owner can withdraw assets from pocket successfully", async () => {
    const {
      program,
      pocketAccount,
      owner,
      baseMintAccount,
      targetMintAccount,
      baseMintVaultAccount,
      targetMintVaultAccount,
      ownerBaseTokenAccount,
      ownerTargetTokenAccount,
      provider
    } = fixtures;

    await program.methods
      .updatePocket({
        status: { closed: {} },
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
      })
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const txId = await program.methods
      .withdraw()
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        pocketTargetTokenVault: targetMintVaultAccount,
        signerBaseTokenAccount: ownerBaseTokenAccount,
        signerTargetTokenAccount: ownerTargetTokenAccount
      })
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.baseTokenBalance.eq(new anchor.BN(0))).to.be.true;
    expect(pocketState.targetTokenBalance.eq(new anchor.BN(0))).to.be.true;

    // expect log
    const transaction = await provider.connection.getParsedTransaction(txId as string, {
      commitment: "confirmed",
    });
    const eventParser = new EventParser(
      program.programId,
      new BorshCoder(program.idl)
    );
    const [event] = eventParser.parseLogs(transaction.meta.logMessages);

    // Expect emitted logs
    expect(event.name).eq("PocketWithdrawn");
    expect(
      (event.data as any).owner.equals(owner.publicKey)
    ).equals(true);
    expect(
      (event.data as any).pocketAddress.equals(pocketAccount)
    ).equals(true);

    expect(
      (event.data as any).baseTokenMintAddress.equals(baseMintAccount)
    ).equals(true);
    expect(
      (event.data as any).baseTokenAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))
    ).equals(true);

    expect(
      (event.data as any).targetTokenMintAddress.equals(targetMintAccount)
    ).equals(true);
    expect(
      (event.data as any).targetTokenAmount.eq(new anchor.BN(0))
    ).equals(true);
  })


  it("[swap] should: pocket operator can trigger swap", async () => {
    // market on devnet
    // https://www.serumexplorer.xyz/market/FdDgZX4vCyKWBPYn8824re6tAsq5ahTWtYkHiGugwox4?network=devnet&address=DEX2Dj46YeRkFnwGpBewZXYuNjm6XzQDoAg9W7NvLvuV
    // // Swaps two tokens on a single A/B market, where A is the base currency
    //     /// and B is the quote currency. This is just a direct IOC trade that
    //     /// instantly settles.
    //     ///
    //     /// When side is "bid", then swaps B for A. When side is "ask", then swaps
    //     /// A for B.
    //     ///
    //     /// Arguments:
    //     ///
    //     /// * `side`              - The direction to swap.
    //     /// * `amount`            - The amount to swap *from*
    //     /// * `min_exchange_rate` - The exchange rate to use when determining
    //     ///    whether the transaction should abort.



  })

});
