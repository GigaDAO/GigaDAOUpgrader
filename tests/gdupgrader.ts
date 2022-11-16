import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gdupgrader } from "../target/types/gdupgrader";

describe("gdupgrader", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Gdupgrader as Program<Gdupgrader>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.upgrade().rpc();
    console.log("Your transaction signature", tx);
  });
});
