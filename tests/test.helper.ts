import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

import {
  createAssociatedTokenAccount,
  createMint,
  mintTo,
} from "@solana/spl-token";
import { AnchorProvider } from "@project-serum/anchor/dist/cjs/provider";

import { IDL } from "../target/types/pocket";
export const getFixtures = async (provider: AnchorProvider) => {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = new anchor.Program(IDL, process.env.PROGRAM_ID);
  const deployer = provider.wallet as anchor.Wallet;

  const pocketId = Keypair.generate().publicKey.toString().slice(0, 24);

  // find the pocket account
  const [pocketRegistry] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("SEED::POCKET::PLATFORM")],
    program.programId
  );

  // find the pocket account
  const [pocketAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::POCKET_SEED"),
      anchor.utils.bytes.utf8.encode(pocketId),
    ],
    program.programId
  );

  let owner: Keypair = Keypair.generate();
  let nonOwner: Keypair = Keypair.generate();

  // Funding signer accounts
  await provider.connection.requestAirdrop(
    owner.publicKey,
    LAMPORTS_PER_SOL * 1
  );

  await provider.connection.requestAirdrop(
    nonOwner.publicKey,
    LAMPORTS_PER_SOL * 1
  );

  // Create mint token and funding token
  const baseMintAccount = await createMint(
    provider.connection,
    deployer.payer,
    deployer.publicKey,
    deployer.publicKey,
    9
  );

  // create associated token account
  const ownerBaseTokenAccount = await createAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    baseMintAccount,
    owner.publicKey
  );

  // create associated token account
  const nonOwnerBaseTokenAccount = await createAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    baseMintAccount,
    nonOwner.publicKey
  );

  const targetMintAccount = await createMint(
    provider.connection,
    deployer.payer,
    deployer.publicKey,
    deployer.publicKey,
    9
  );

  // create associated token account
  const ownerTargetTokenAccount = await createAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    owner.publicKey
  );

  // create associated token account
  const nonOwnerTargetTokenAccount = await createAssociatedTokenAccount(
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
    ownerBaseTokenAccount,
    deployer.publicKey,
    LAMPORTS_PER_SOL * 100
  );

  // funding token
  await mintTo(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    ownerTargetTokenAccount,
    deployer.publicKey,
    LAMPORTS_PER_SOL * 100
  );

  const [baseMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      baseMintAccount.toBytes(),
    ],
    program.programId
  );

  const [targetMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      targetMintAccount.toBytes(),
    ],
    program.programId
  );

  return {
    program,
    provider,
    deployer,
    pocketRegistry,
    pocketAccount,
    pocketId,
    owner,
    ownerBaseTokenAccount,
    ownerTargetTokenAccount,
    nonOwner,
    nonOwnerBaseTokenAccount,
    nonOwnerTargetTokenAccount,
    baseMintAccount,
    targetMintAccount,
    baseMintVaultAccount,
    targetMintVaultAccount,
  };
};
