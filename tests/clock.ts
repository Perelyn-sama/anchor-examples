import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Clock } from "../target/types/clock";

describe("clock", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Clock as Program<Clock>;

  it("get clock", async () => {
    const clockStateKeyPair = anchor.web3.Keypair.generate();

    // Add your test here.
    const tx = await program.methods.getClock().accounts({
      user: provider.wallet.publicKey,
      clockState:clockStateKeyPair.publicKey,
    }).signers([clockStateKeyPair]).rpc();

    console.log("Your transaction signature", tx);

    let clockState = await program.account.clockState.fetch(clockStateKeyPair.publicKey);
    console.log(clockState);

  });
});
