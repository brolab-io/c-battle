import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CBattle } from "../target/types/c_battle";
import { expect } from "chai";

describe("c-battle", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const payer = provider.wallet.payer;
  anchor.setProvider(provider);

  const program = anchor.workspace.cBattle as Program<CBattle>;
  const configParams = {
    powerUpBaseCost: new anchor.BN(1000),
    repawnCost: new anchor.BN(500),
    maxPlayersPerArena: 10,
    battleDurationSecs: 300,
    eliminationReward: new anchor.BN(10000),
  }

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("battle-config")],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(
      configParams.powerUpBaseCost,
      configParams.repawnCost,
      configParams.maxPlayersPerArena,
      configParams.battleDurationSecs,
      configParams.eliminationReward,
    ).rpc();
    console.log("Your transaction signature", tx);
    const configAccount = await program.account.battleConfig.fetch(configPda);
    expect(configAccount.authority.equals(payer.publicKey)).to.be.true;
    expect(configAccount.powerUpBaseCost.eq(configParams.powerUpBaseCost)).to.be.true;
    expect(configAccount.repawnCost.eq(configParams.repawnCost)).to.be.true;
    expect(configAccount.maxPlayersPerArena).to.eql(configParams.maxPlayersPerArena);
    expect(configAccount.battleDurationSecs).to.eql(configParams.battleDurationSecs);
    expect(configAccount.eliminationReward.eq(configParams.eliminationReward)).to.be.true;
  });
});
