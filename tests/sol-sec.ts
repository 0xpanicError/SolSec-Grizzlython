import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolSec } from "../target/types/sol_sec";

describe("sol-sec", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolSec as Program<SolSec>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
