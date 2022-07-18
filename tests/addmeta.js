const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram, PublicKey, SYSVAR_RENT_PUBKEY } = anchor.web3;
const { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } = require("@solana/spl-token");
const { findMetadataPda } = require("@metaplex-foundation/js");

const CTRLSEED = 'CTRLv1';
const ADMINACC = "Ec3nzEVcQ7jxgJWoHF9eECa2ysyjK95h6agNQVesNXnK";  // Replace with YOUR wallet address

describe("addmeta", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  usr = provider.wallet.publicKey;
  console.log("Endpoint:", provider.connection.rpcEndpoint);
  const program = anchor.workspace.Addmeta;

  it("Gets some CTRL", async () => {

    const [ctrlPDA, bump] = await PublicKey.findProgramAddress([anchor.utils.bytes.utf8.encode(CTRLSEED)], program.programId);
    console.log("GetCTRL: ctrlPDA is [", ctrlPDA.toString(), "], bump ", bump);

    let receiver = usr;
    let assocTokAcct = await getAssociatedTokenAddress(
      ctrlPDA,
      receiver,
      true, // allowOwnerOffCurve
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );
    console.log("GetCTRL: AssocTokAcct: ", assocTokAcct.toString());

    let bigAmount = new anchor.BN(888 * 1000000);

    await program.methods
      .mintCtrl(bump, bigAmount)
      .accounts({
        mint: ctrlPDA,
        destination: assocTokAcct,
        payer: usr,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }).rpc();

    console.log("GetCTRL done.");
    assert.ok(true);
  });

  it("Sets the token metadata", async () => {
    const [ctrlPDA, bump] = await PublicKey.findProgramAddress([anchor.utils.bytes.utf8.encode(CTRLSEED)], program.programId);
    console.log("TokMeta: ctrlPDA is [", ctrlPDA.toString(), "], bump ", bump);
    const metadataPDA = await findMetadataPda(ctrlPDA);
    console.log("TokMeta: metadataPDA is [", metadataPDA.toString(), "]");
    const adm = new PublicKey(ADMINACC);
    console.log("TokMeta: usr=", usr.toString(), "adm=", adm.toString())
    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    console.log("TokMeta: METADATA_PROGRAM_ID:", METADATA_PROGRAM_ID.toString());

    await program.methods
      .tokMeta(bump)
      .accounts({
        mint: ctrlPDA,
        mintauth: ctrlPDA,
        payer: usr,
        updauth: adm,
        metadataPda: metadataPDA,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METADATA_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

      console.log("TokMeta done.");
    assert.ok(true);
  });


 });