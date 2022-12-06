import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { UnstakeMySol } from "../target/types/unstake_my_sol";

describe("unstake-my-sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.UnstakeMySol as Program<UnstakeMySol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
