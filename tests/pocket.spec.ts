import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { expect } from "chai";
import { getAccount } from "@solana/spl-token";
import { getFixtures } from "./test.helper";
import { BorshCoder, EventParser } from "@project-serum/anchor";

describe("pocket", async () => {
  let fixtures: Awaited<ReturnType<typeof getFixtures>>;

  before(async () => {
    fixtures = await getFixtures(anchor.AnchorProvider.env());
  });

  it("[create_pocket] should: anyone can create their pocket", async () => {
    const {
      provider,
      pocketId,
      program,
      targetMintAccount,
      baseMintAccount,
      pocketAccount,
      owner,
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

    const txId = await program.methods
      .createPocket(pocketData)
      .accounts({
        pocket: pocketAccount,
        signer: owner.publicKey,
      })
      .signers([owner])
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));

    const pocket = await program.account.pocket.fetch(pocketAccount);

    // @ts-ignore
    expect(!!pocket.status.active).to.be.true;
    // @ts-ignore
    expect(!!pocket.side.buy).to.be.true;
    expect(pocket.name === pocketData.name).to.be.true;
    expect(pocket.owner.equals(owner.publicKey)).to.be.true;
    expect(pocket.id === pocketData.id).to.be.true;
    expect(pocket.startAt.eq(pocketData.startAt)).to.be.true;
    expect(pocket.quoteTokenMintAddress.equals(pocketData.quoteTokenAddress))
      .to.be.true;
    expect(pocket.baseTokenMintAddress.equals(pocketData.baseTokenAddress)).to
      .be.true;
    // @ts-ignore
    expect(pocket.stopConditions.length === 0).to.be.true;
    expect(!!pocket.buyCondition).to.be.false;
    expect(pocket.frequency.hours.eq(pocketData.frequency.hours)).to.be.true;
    expect(pocket.batchVolume.eq(pocketData.batchVolume)).to.be.true;
    expect(pocket.marketKey.equals(pocketData.marketKey)).to.be.true;


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
    expect(event.name).eq('PocketCreated');
    expect(
      (event.data as any).owner.equals(owner.publicKey)
    ).equals(true);
    expect(
      (event.data as any).pocketAddress.equals(pocketAccount)
    ).equals(true);
    expect(
      (event.data as any).name === pocketData.name
    ).equals(true);
  });

  it("[create_token_vault] should: pocket owner can create token vault successfully", async () => {
    const {
      program,
      provider,
      baseMintAccount,
      targetMintAccount,
      pocketAccount,
      targetMintVaultAccount,
      baseMintVaultAccount,
      owner,
    } = fixtures;

    expect(!!(await provider.connection.getAccountInfo(targetMintVaultAccount)))
      .to.be.false;
    expect(!!(await provider.connection.getAccountInfo(baseMintVaultAccount)))
      .to.be.false;

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

    const tx = new Transaction().add(...inx);
    const txId = await provider.sendAndConfirm(tx, [owner], {commitment: "confirmed"}).catch((e) => console.log(e));

    expect(!!(await provider.connection.getAccountInfo(targetMintVaultAccount)))
      .to.be.true;
    expect(
      (
        await getAccount(provider.connection, targetMintVaultAccount)
      ).owner.equals(pocketAccount)
    ).to.be.true;

    expect(!!(await provider.connection.getAccountInfo(baseMintVaultAccount)))
      .to.be.true;
    expect(
      (
        await getAccount(provider.connection, targetMintVaultAccount)
      ).owner.equals(pocketAccount)
    ).to.be.true;

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
    expect(event.name).eq('VaultCreated');
    expect(
      (event.data as any).actor.equals(owner.publicKey)
    ).equals(true);
    expect(
      (event.data as any).mintAccount.equals(baseMintAccount)
    ).equals(true);
    expect(
      (event.data as any).authority.equals(pocketAccount)
    ).equals(true);
    expect(
      (event.data as any).associatedAccount.equals(baseMintVaultAccount)
    ).equals(true);
  });

  it("[pause_pocket] should: owner should pause pocket successfully", async () => {
    const { provider, program, pocketAccount, owner } = fixtures;

    const txId = await program.methods
      .updatePocket({
        status: { paused: {} },
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
      })
      .signers([owner])
      .rpc({commitment: "confirmed"})
      .catch((e) => console.log(e));

    const pocket = await program.account.pocket.fetch(pocketAccount);

    expect(!!pocket.status.paused).to.be.true;

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
    expect(event.name).eq('PocketUpdated');
    expect(
      (event.data as any).actor.equals(owner.publicKey)
    ).equals(true);
    expect(
      (event.data as any).pocketAddress.equals(pocketAccount)
    ).equals(true);
    expect(
      !!(event.data as any).status.paused
    ).equals(true);
  });

  it("[pause_pocket] should: owner should not pause pocket that was already paused", async () => {
    const { program, pocketAccount, owner } = fixtures;

    await program.methods
      .updatePocket({
        status: { paused: {} },
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
      })
      .signers([owner])
      .rpc()
      .then(() => {
        throw new Error("ShouldFailed");
      })
      .catch((e) => expect(e.toString().includes("ShouldFailed")).to.be.false);
  });

  it("[restart_pocket] should: owner can restart the paused pocket successfully", async () => {
    const { program, pocketAccount, owner } = fixtures;

    await program.methods
      .updatePocket({
        status: { active: {} },
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
      })
      .signers([owner])
      .rpc()
      .catch((e) => console.log(e));

    const pocket = await program.account.pocket.fetch(pocketAccount);

    expect(!!pocket.status.active).to.be.true;
  });

  it("[close_pocket] should: owner can close pocket successfully", async () => {
    const { program, pocketAccount, owner } = fixtures;

    await program.methods
      .updatePocket({
        status: { closed: {} },
      })
      .accounts({
        signer: owner.publicKey,
        pocket: pocketAccount,
      })
      .signers([owner])
      .rpc()
      .catch((e) => console.log(e));

    const pocket = await program.account.pocket.fetch(pocketAccount);

    expect(!!pocket.status.closed).to.be.true;
  });
});
