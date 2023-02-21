import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, SendTransactionError } from "@solana/web3.js";
import { expect } from "chai";

import { IDL } from "../target/types/pocket";

describe("pocket_registry", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(IDL, process.env.PROGRAM_ID);
  const deployer = provider.wallet as anchor.Wallet;

  // find the pocket account
  const [pocketRegistry] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("SEED::POCKET::PLATFORM")],
    program.programId
  );

  const operator = Keypair.generate().publicKey;

  before(async () => {
    // Initialize first
    await program.methods
      .initialize({
        operators: [operator],
      })
      .accounts({
        pocketRegistry,
        owner: deployer.publicKey,
      })
      .signers([deployer.payer])
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));
  });

  it("[initialize_swap_program] should: deployer initializes pocket registry successfully", async () => {
    const state = await program.account.pocketPlatformRegistry.fetch(
      pocketRegistry
    );

    // Expect conditions
    expect(state.owner.equals(deployer.publicKey));
    expect(state.wasInitialized).equals(true);
    expect(state.operators.length).equals(1);
    expect(state.operators[0].equals(operator)).to.be.true;

    // @ts-ignore
    expect(state.allowedMintAccounts.length).equals(0);
  });

  it("[initialize_swap_program] should: deployer fails to re-initialize the pocket registry", async () => {
    try {
      await program.methods
        .initialize({
          operators: [operator],
        })
        .accounts({
          pocketRegistry: pocketRegistry,
          owner: deployer.publicKey,
        })
        .signers([deployer.payer])
        .rpc({ commitment: "confirmed" });

      throw new Error("Should be failed");
    } catch (e) {
      expect(e instanceof SendTransactionError).to.be.true;
    }
  });

  it("[update_operator] should: deployer can update operators list", async () => {
    const newOperator = Keypair.generate().publicKey;

    await program.methods
      .updatePocketRegistry({
        operators: [newOperator],
      })
      .accounts({
        pocketRegistry,
        owner: deployer.publicKey,
      })
      .signers([deployer.payer])
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));

    const pocketRegistryAccount =
      await program.account.pocketPlatformRegistry.fetch(pocketRegistry);

    expect(pocketRegistryAccount.operators.length).eq(1);
    expect(pocketRegistryAccount.operators[0].equals(newOperator)).to.be.true;
  });
});
