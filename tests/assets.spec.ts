import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { expect } from "chai";
import { getAccount } from "@solana/spl-token";
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
        pocketRegistry,
      })
      .signers([owner])
      .postInstructions(inx)
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));
  });

  it("[deposit] should: owner can deposit assets to pocket successfully", async () => {
    const {
      program,
      pocketAccount,
      owner,
      baseMintVaultAccount,
      ownerBaseTokenAccount,
    } = fixtures;

    await program.methods
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
      .rpc()
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.baseTokenBalance.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
  });

  it("[withdraw] should: owner can withdraw assets from pocket successfully", async () => {
    const {
      program,
      pocketAccount,
      owner,
      baseMintVaultAccount,
      targetMintVaultAccount,
      ownerBaseTokenAccount,
      ownerTargetTokenAccount
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
      .rpc()
      .catch((e) => console.log(e));

    await program.methods
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
      .rpc()
      .catch((e) => console.log(e));

    const pocketState = await program.account.pocket.fetch(pocketAccount);

    expect(pocketState.totalDepositAmount.eq(new anchor.BN(LAMPORTS_PER_SOL * 2))).to.be.true;
    expect(pocketState.baseTokenBalance.eq(new anchor.BN(0))).to.be.true;
  })
});
