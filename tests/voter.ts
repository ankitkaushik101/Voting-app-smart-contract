import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voter } from "../target/types/voter";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("voter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voter as Program<Voter>;
  const user = anchor.AnchorProvider.local().wallet;
  const voter = Keypair.generate();

  console.log(voter.publicKey.toBase58());
  console.log(user.publicKey.toBase58());

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        partyJkl: voter.publicKey,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([voter])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // increament the account

  it("Vote account", async () => {
    if (!voter) {
      throw new Error("Account not initialized");
    }
    const txn = await program.methods
      .upvote()
      .accounts({
        partyJkl: voter.publicKey,
      })
      .rpc();
    await program.account.jkl.fetch(voter.publicKey);
    console.log("Your transaction signature", txn);
  });
});
