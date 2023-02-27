import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

import {
  createMint, getOrCreateAssociatedTokenAccount,
  mintTo
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
      anchor.utils.bytes.utf8.encode(pocketId)
    ],
    program.programId
  );

  let nonOwner: Keypair = Keypair.generate();
  let owner: Keypair = Keypair.generate();
  let operator: Keypair = Keypair.generate();

  // Funding signer accounts
  await provider.connection.requestAirdrop(
    operator.publicKey,
    LAMPORTS_PER_SOL * 2
  );

  // Funding signer accounts
  await provider.connection.requestAirdrop(
    owner.publicKey,
    LAMPORTS_PER_SOL * 2
  );

  await provider.connection.requestAirdrop(
    nonOwner.publicKey,
    LAMPORTS_PER_SOL * 2
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
  const ownerBaseTokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    baseMintAccount,
    owner.publicKey
  );

  // create associated token account
  const nonOwnerBaseTokenAccount = await getOrCreateAssociatedTokenAccount(
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
  const ownerTargetTokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    owner.publicKey
  );

  // create associated token account
  const nonOwnerTargetTokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    nonOwner.publicKey
  );

  // wont mint if use existed mints
  // funding token
  await mintTo(
    provider.connection,
    deployer.payer,
    baseMintAccount,
    ownerBaseTokenAccount.address,
    deployer.publicKey,
    LAMPORTS_PER_SOL * 100
  );

  // funding token
  await mintTo(
    provider.connection,
    deployer.payer,
    targetMintAccount,
    ownerTargetTokenAccount.address,
    deployer.publicKey,
    LAMPORTS_PER_SOL * 100
  );

  const [baseMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      baseMintAccount.toBytes()
    ],
    program.programId
  );

  const [targetMintVaultAccount] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("SEED::POCKET::TOKEN_VAULT_SEED"),
      pocketAccount.toBytes(),
      targetMintAccount.toBytes()
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
    operator,
    owner,
    ownerBaseTokenAccount,
    ownerTargetTokenAccount,
    nonOwner,
    nonOwnerBaseTokenAccount,
    nonOwnerTargetTokenAccount,
    baseMintAccount,
    targetMintAccount,
    baseMintVaultAccount,
    targetMintVaultAccount
  };
};
