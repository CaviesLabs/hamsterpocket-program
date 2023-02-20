import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { expect } from "chai";
import {getAccount} from  '@solana/spl-token'
import { getFixtures } from "./test.helper";

describe("create_pocket", async () => {
  let fixtures: Awaited<ReturnType<typeof getFixtures>>;

  before(async () => {
    fixtures = await getFixtures(anchor.AnchorProvider.env());
  });

  it("[create_pocket] should: anyone can create their pocket", async () => {
    const {
      pocketId,
      program,
      pocketRegistry,
      targetMintAccount,
      baseMintAccount,
      pocketAccount,
      owner,
    } = fixtures;

    const pocketData = {
      marketKey: Keypair.generate().publicKey,
      id: pocketId,
      targetTokenAddress: targetMintAccount,
      baseTokenAddress: baseMintAccount,
      stopConditions: [],
      buyCondition: null,
      startAt: new anchor.BN(new Date().getTime().toString()),
      batchVolume: new anchor.BN((LAMPORTS_PER_SOL * 10).toString()),
      name: "pocket name",
      frequency: { hours: new anchor.BN(1) },
    };

    await program.methods
      // @ts-ignore
      .createPocket(pocketData)
      .accounts({
        pocket: pocketAccount,
        signer: owner.publicKey,
        pocketRegistry,
      })
      .signers([owner])
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));

    const pocket = await program.account.pocket.fetch(pocketAccount);

    // @ts-ignore
    expect(!!pocket.status.active).to.be.true;
    expect(pocket.name === pocketData.name).to.be.true;
    expect(pocket.id === pocketData.id).to.be.true;
    expect(pocket.startAt.eq(pocketData.startAt)).to.be.true;
    expect(pocket.marketKey.equals(pocketData.marketKey)).to.be.true;
    expect(pocket.targetTokenAddress.equals(pocketData.targetTokenAddress)).to
      .be.true;
    expect(pocket.baseTokenAddress.equals(pocketData.baseTokenAddress)).to.be
      .true;
    // @ts-ignore
    expect(pocket.stopConditions.length === 0).to.be.true;
    expect(!!pocket.buyCondition).to.be.false;
    expect(pocket.frequency.hours.eq(pocketData.frequency.hours)).to.be.true;
    expect(pocket.batchVolume.eq(pocketData.batchVolume)).to.be.true;
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

    expect(!!await provider.connection.getAccountInfo(targetMintVaultAccount)).to.be.false;
    expect(!!await provider.connection.getAccountInfo(baseMintVaultAccount)).to.be.false;

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
    await provider.sendAndConfirm(tx, [owner]).catch((e) => console.log(e));

    expect(!!await provider.connection.getAccountInfo(targetMintVaultAccount)).to.be.true;
    expect((await getAccount(provider.connection, targetMintVaultAccount)).owner.equals(pocketAccount)).to.be.true;

    expect(!!await provider.connection.getAccountInfo(baseMintVaultAccount)).to.be.true;
    expect((await getAccount(provider.connection, targetMintVaultAccount)).owner.equals(pocketAccount)).to.be.true;
  });
});
