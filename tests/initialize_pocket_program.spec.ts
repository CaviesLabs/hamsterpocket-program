import * as anchor from "@project-serum/anchor";
import { BN } from "@project-serum/anchor";
import { Keypair, PublicKey, SendTransactionError } from "@solana/web3.js";
import { expect } from "chai";

import { IDL } from "../target/types/pocket";

describe("initialize_pocket_program", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(IDL, process.env.PROGRAM_ID);
  const deployer = provider.wallet as anchor.Wallet;

  // find the pocket account
  const [pocketAccount] = PublicKey.findProgramAddressSync(
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
        pocketRegistry: pocketAccount,
        owner: deployer.publicKey,
      })
      .signers([deployer.payer])
      .rpc({ commitment: "confirmed" })
      .catch((e) => console.log(e));
  });

  it("[initialize_swap_program] should: deployer initializes pocket registry successfully", async () => {
    const state = await program.account.pocketPlatformRegistry.fetch(
      pocketAccount
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
          operators: [operator]
        })
        .accounts({
          pocketRegistry: pocketAccount,
          owner: deployer.publicKey,
        })
        .signers([deployer.payer])
        .rpc({ commitment: "confirmed" });

      throw new Error("Should be failed");
    } catch (e) {
      expect(e instanceof SendTransactionError).to.be.true;
    }
  });
});
