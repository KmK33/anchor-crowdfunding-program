import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai"
import { Web3 } from "../target/types/web3";

describe("crowdfunding", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Web3 as Program<Web3>;

  let campaignKeypair = anchor.web3.Keypair.generate();
  const campaignAmount = 1000;

  it("Creates a campaign", async () => {

    //Call our create campaign function
    await program.methods
      .createCampaign(
        "Campaign Title",
        "Campaign Description",
        new anchor.BN(campaignAmount),
        new anchor.BN(Math.floor(Date.now() / 1000) + 86400), // 1 day from now
        "Image URL"
      )
      .accounts({
        campaign: campaignKeypair.publicKey,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([campaignKeypair]) // Sign with the newly generated Keypair
      .rpc();


    //running checks
    const campaign = await program.account.campaign.fetch(campaignKeypair.publicKey);
    assert.equal(campaign.title, "Campaign Title");
    assert.equal(campaign.description, "Campaign Description");
    assert.equal(campaign.target.toNumber(), campaignAmount);
    assert.equal(campaign.amountCollected.toNumber(), 0);
    assert.equal(campaign.image, "Image URL");
  });

  it("Donates to the campaign", async () => {

    //Getting the balance of the initial wallet
    const initialBalance = await provider.connection.getBalance(provider.wallet.publicKey);
    const donationAmount = 500;


    //calling of function
    await program.methods
      .donateToCampaign(new anchor.BN(donationAmount))
      .accounts({
        campaign: campaignKeypair.publicKey,
        donator: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    //Performing checks
    const campaign = await program.account.campaign.fetch(campaignKeypair.publicKey);
    assert.equal(campaign.amountCollected.toNumber(), donationAmount);
    assert.equal(campaign.donations.length, 1);
    assert.equal(campaign.donators.length, 1);
    assert.equal(campaign.donations[0].toNumber(), donationAmount);
    assert.equal(campaign.donators[0].toBase58(), provider.wallet.publicKey.toBase58());

    const finalBalance = await provider.connection.getBalance(provider.wallet.publicKey);

    // Allow for a small variance in the balance due to transaction fees
    const balanceDifference = initialBalance - finalBalance;
    assert.isTrue(
      balanceDifference >= donationAmount && balanceDifference <= donationAmount + 10000,
      `Balance difference ${balanceDifference} not within expected range`
    );
  });
});
