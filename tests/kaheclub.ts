import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Kaheclub } from "../target/types/kaheclub";

describe("kaheclub", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Kaheclub as Program<Kaheclub>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
