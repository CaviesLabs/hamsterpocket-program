import * as anchor from "@project-serum/anchor";
import { BN } from "@project-serum/anchor";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

import { IDL } from "../target/types/pocket";
import { createAssociatedTokenAccount, createMint, mintTo } from "@solana/spl-token";

describe("initialize_pocket_program", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(IDL, process.env.PROGRAM_ID)
  const deployer = provider.wallet as anchor.Wallet;

  // find the pocket account
  const [pocketAccount] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("SEED::POCKET::PLATFORM")],
    program.programId
  );

  let baseMintAccount: PublicKey;
  let baseMintVaultAccount: PublicKey;

  let targetMintAccount: PublicKey;
  let targetMintVaultAccount: PublicKey;

  let owner: Keypair = Keypair.generate();
  let ownerBaseTokenAccount: PublicKey;
  let ownerTargetTokenAccount: PublicKey;

  let nonOwner: Keypair = Keypair.generate();
  let nonOwnerBaseTokenAccount: PublicKey;
  let nonOwnerTargetTokenAccount: PublicKey;

  before(async () => {
    // Funding signer accounts
    await provider.connection.requestAirdrop(
      owner.publicKey,
      LAMPORTS_PER_SOL * 100
    );

    await provider.connection.requestAirdrop(
      nonOwner.publicKey,
      LAMPORTS_PER_SOL * 100
    );

    // Create mint token and funding token
    baseMintAccount = await createMint(
      provider.connection,
      deployer.payer,
      deployer.publicKey,
      deployer.publicKey,
      9
    );

    // create associated token account
    ownerBaseTokenAccount= await createAssociatedTokenAccount(
      provider.connection,
      deployer.payer,
      baseMintAccount,
      owner.publicKey
    );

    // create associated token account
    nonOwnerBaseTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      deployer.payer,
      baseMintAccount,
      nonOwner.publicKey
    );

    targetMintAccount = await createMint(
      provider.connection,
      deployer.payer,
      deployer.publicKey,
      deployer.publicKey,
      9
    );

    // create associated token account
    ownerTargetTokenAccount= await createAssociatedTokenAccount(
      provider.connection,
      deployer.payer,
      targetMintAccount,
      owner.publicKey
    );

    // create associated token account
    nonOwnerTargetTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      deployer.payer,
      targetMintAccount,
      nonOwner.publicKey
    );

    // funding token
    await mintTo(
      provider.connection,
      deployer.payer,
      baseMintAccount,
      owner.publicKey,
      deployer.publicKey,
      LAMPORTS_PER_SOL * 100
    );

    // funding token
    await mintTo(
      provider.connection,
      deployer.payer,
      targetMintAccount,
      owner.publicKey,
      deployer.publicKey,
      LAMPORTS_PER_SOL * 100
    );
  });

});
