import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
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
      quoteTokenAddress: targetMintAccount,
      baseTokenAddress: baseMintAccount,
      stopConditions: [],
      buyCondition: null,
      startAt: new anchor.BN(new Date().getTime().toString()),
      batchVolume: new anchor.BN((LAMPORTS_PER_SOL * 10).toString()),
      name: "pocket name",
      frequency: { hours: new anchor.BN(1) },
      marketKey: Keypair.generate().publicKey,
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
      .createPocket(pocketData)
      .accounts({
        pocket: pocketAccount,
        signer: owner.publicKey,
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
      targetMintVaultAccount,
      ownerBaseTokenAccount,
      ownerTargetTokenAccount
    } = fixtures;

    const txId = await program.methods
      .deposit({
        depositAmount: new anchor.BN(LAMPORTS_PER_SOL * 2),
        mode: {base: {}}
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        pocketQuoteTokenVault: targetMintVaultAccount,
        signerBaseTokenAccount: ownerBaseTokenAccount.address,
        signerQuoteTokenAccount: ownerTargetTokenAccount.address,
      })
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalBaseDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.totalQuoteDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 0))).to.be.true;
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

    const txId = await program.methods
      .withdraw()
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
        pocketBaseTokenVault: baseMintVaultAccount,
        pocketQuoteTokenVault: targetMintVaultAccount,
        signerBaseTokenAccount: ownerBaseTokenAccount.address,
        signerQuoteTokenAccount: ownerTargetTokenAccount.address
      })
      .preInstructions([
        await program.methods
          .updatePocket({
            status: { closed: {} },
          })
          .accounts({
            signer: owner.publicKey,
            pocket: pocketAccount,
          })
          .instruction()
      ])
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalBaseDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.totalQuoteDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 0))).to.be.true;
    expect(pocketState.baseTokenBalance.eq(new anchor.BN(0))).to.be.true;
    expect(pocketState.quoteTokenBalance.eq(new anchor.BN(0))).to.be.true;
    expect(!!pocketState.status.withdrawn).to.be.true;

    // expect log
    const transaction = await provider.connection.getParsedTransaction(txId as string, {
      commitment: "confirmed",
    });
    const eventParser = new EventParser(
      program.programId,
      new BorshCoder(program.idl)
    );
    const [,event] = eventParser.parseLogs(transaction.meta.logMessages);

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
      (event.data as any).quoteTokenMintAddress.equals(targetMintAccount)
    ).equals(true);
    expect(
      (event.data as any).quoteTokenAmount.eq(new anchor.BN(0))
    ).equals(true);
  });

  it("[close_pocket_accounts] should: owner should close pocket accounts and claim fee back successfully", async () => {
    const {
      program,
      pocketAccount,
      owner,
      baseMintVaultAccount,
      targetMintVaultAccount,
      provider
    } = fixtures;

    const beforeClosedBalance = await provider.connection.getBalance(owner.publicKey);

    await program.methods.closePocketAccounts().accounts({
      signer: owner.publicKey,
      pocket: pocketAccount,
      pocketBaseTokenVault: baseMintVaultAccount,
      pocketQuoteTokenVault: targetMintVaultAccount
    })
      .signers([owner])
      .rpc({commitment: 'confirmed'})
      .catch((e) => console.log(e));

    const afterClosedBalance = await provider.connection.getBalance(owner.publicKey);

    /**
     * @dev Expect balance
     */
    expect(afterClosedBalance).gt(beforeClosedBalance);
  });
});
