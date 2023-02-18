import {
  AddressLookupTableAccount,
  Keypair,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import type { AnchorProvider } from "@project-serum/anchor/dist/cjs/provider";

export class V0transactionProvider {
  /**
   * @dev Send and confirm v0 transaction
   * @param provider
   * @param instructions
   * @param signer
   */
  public async sendAndConfirmV0Transaction(
    provider: AnchorProvider,
    instructions: TransactionInstruction[],
    signer: Keypair
  ): Promise<string> {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    const lookupMessage = new TransactionMessage({
      payerKey: signer.publicKey,
      recentBlockhash: latestBlockHash.blockhash,
      instructions: instructions,
    }).compileToV0Message();

    const lookupTransaction = new VersionedTransaction(lookupMessage);
    lookupTransaction.sign([signer]);

    const txid = await provider.connection.sendRawTransaction(
      lookupTransaction.serialize()
    );

    await provider.connection.confirmTransaction(
      {
        signature: txid,
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      },
      "finalized"
    );

    return txid;
  }

  /**
   * @dev Send and confirm V0 transaction with Address lookup table.
   * @param provider
   * @param instructions
   * @param lookupTableAccounts
   * @param signer
   */
  public async sendAndConfirmV0TransactionWithALT(
    provider: AnchorProvider,
    instructions: TransactionInstruction[],
    lookupTableAccounts: AddressLookupTableAccount[],
    signer: Keypair
  ): Promise<string> {
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    const lookupMessage = new TransactionMessage({
      payerKey: signer.publicKey,
      recentBlockhash: latestBlockHash.blockhash,
      instructions: instructions,
    }).compileToV0Message(lookupTableAccounts);

    const lookupTransaction = new VersionedTransaction(lookupMessage);
    lookupTransaction.sign([signer]);

    const txid = await provider.connection.sendRawTransaction(
      lookupTransaction.serialize()
    );
    await provider.connection.confirmTransaction(
      {
        signature: txid,
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      },
      "confirmed"
    );
    return txid;
  }
}
