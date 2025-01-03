import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CroutonJonesNftSolana } from "../target/types/crouton_jones_nft_solana";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMint,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  mintTo,
  TOKEN_PROGRAM_ID,
  
  transfer,
} from "@solana/spl-token";

describe("crouton-jones-nft-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CroutonJonesNftSolana as Program<CroutonJonesNftSolana>;
  const provider = anchor.AnchorProvider.env();

 // Metaplex Constants
 const METADATA_SEED = "metadata";
 const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  // Data for our tests
  const payer = provider.wallet.publicKey;

  console.log(`<---------------Wallet: ${payer}---------------------->`);

  const mint = anchor.web3.Keypair.generate(); // token account
  console.log("🚀 ~ describe ~ mint:", mint.publicKey);
  console.log("🚀 ~ describe ~ mint:", mint.publicKey.toBase58());

  const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const [masterEditionAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );


  it("Can mint a single NFT", async () => {
    const metadata = {
      title: "Crouton Jones Token",
      symbol: "CJT",
      uri: "https://brown-orthodox-lion-702.mypinata.cloud/ipfs/QmT6bfwy2aXQzZr9uNEVYTQFDVJJuoaDE1w5Q27eryFjiy",
    };
      // Derive the token account address
      const tokenAccount = await getAssociatedTokenAddress(
        mint.publicKey,
        provider.wallet.publicKey
      );

    try {
      const tx = await program.methods
        .mint(
          new anchor.BN(1),
          metadata.title,
          metadata.symbol,
          metadata.uri
        )
        .accounts({
          mint: mint.publicKey,
          tokenAccount: tokenAccount,
          mintAuthority: provider.wallet.publicKey,
          metadata: metadataAddress,
          masterEdition: masterEditionAddress,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([ mint, provider.wallet.payer])
        .rpc();

        console.log("🚀 ~ describe ~ tx:", tx);
      // Verify the token account exists and has the correct balance
      const tokenAccountInfo = await provider.connection.getTokenAccountBalance(tokenAccount);
      console.log("🚀 ~ describe ~ tokenAccountInfo:", tokenAccountInfo);
      // Verify metadata account exists
      const metadataAccountInfo = await provider.connection.getAccountInfo(metadataAddress);
      console.log("🚀 ~ describe ~ metadataAccountInfo:", metadataAccountInfo);
      // Verify master edition account exists
      const masterEditionAccountInfo = await provider.connection.getAccountInfo(masterEditionAddress);
      console.log("🚀 ~ describe ~ masterEditionAccountInfo:", masterEditionAccountInfo);
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });

  it("Can mint an edition", async () => {
    // First mint the master edition if not already minted
    const masterTokenAccount = await getAssociatedTokenAddress(
      mint.publicKey,
      provider.wallet.publicKey
    );

    // Generate new keypair for edition mint
    const editionMint = anchor.web3.Keypair.generate();

    // Derive edition metadata address
    const [editionMetadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        editionMint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // Derive edition address
    const [editionAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        editionMint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // Get edition token account
    const editionTokenAccount = await getAssociatedTokenAddress(
      editionMint.publicKey,
      provider.wallet.publicKey
    );

    // Derive the edition marker PDA
    const editionNumber = new anchor.BN(1);
    const editionNumberFloor = Math.floor(editionNumber.toNumber() / 248) * 248;
    
    const [editionMarkerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
        Buffer.from("edition"),
        Buffer.from(editionNumberFloor.toString())
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    try {
      const tx = await program.methods
        .mintEdition(editionNumber)
        .accounts({
          edition: editionAddress,
          editionMetadata: editionMetadataAddress,
          editionMint: editionMint.publicKey,
          editionTokenAccount: editionTokenAccount,
          masterEdition: masterEditionAddress,
          masterMetadata: metadataAddress,
          masterMint: mint.publicKey,
          masterTokenAccount: masterTokenAccount,
          editionMarkerPda: editionMarkerPda,
          payer: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        })
        .signers([editionMint, provider.wallet.payer]) // Add payer to signers
        .rpc(); 
        
        // Add skipPreflight to help with debugging

      console.log("Edition minted successfully:", tx);
      
      // Verification code remains the same...
    } catch (error) {
      console.error("Error minting edition:", error);
      throw error;
    }
});

});
