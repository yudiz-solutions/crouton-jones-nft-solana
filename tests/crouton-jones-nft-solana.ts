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
  const metadata = {
    name: "Crouton Jones Token",
    symbol: "CJT",
    uri: "https://brown-orthodox-lion-702.mypinata.cloud/ipfs/QmT6bfwy2aXQzZr9uNEVYTQFDVJJuoaDE1w5Q27eryFjiy",
    decimals: 9,
    tokenId: new anchor.BN(1)
  };

  const mint = anchor.web3.Keypair.generate(); // token account
  console.log("🚀 ~ describe ~ mint:", mint.publicKey);
  console.log("🚀 ~ describe ~ mint:", mint.publicKey.toBase58());
 
  const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(METADATA_SEED),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const [tokenHolder] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('token_holder'),
    ],
    program.programId
  );

  it("Mint token", async () => {

    const info = await program.provider.connection.getAccountInfo(mint.publicKey);

    console.log(info)
    if (info) {
      return; // Do not attempt to initialize if already initialized
    }
    console.log("<--------------- Attempting to initialize.---------------------->");
 
    const destination = getAssociatedTokenAddressSync(
      mint.publicKey,
      tokenHolder,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    console.log("🚀 ~ it ~ destination/ATA:", destination);

    const context = {
      metadata: metadataAddress,
      mint: mint.publicKey,
      tokenHolder: tokenHolder, // PDA
      destination:destination, // ATA 
      payer,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    };

    const tx = await program.methods
      .initToken(metadata.name, metadata.symbol, metadata.uri, new anchor.BN('1')) //4,000,000,000 with 9 decimals
      .accounts(context).signers([provider.wallet.payer, mint])
      .rpc();

      console.log("<--------------- Token Mint successfully.---------------------->");
       
      let initialBalance: number;
      try {
        const balance = (await provider.connection.getTokenAccountBalance(destination))
        initialBalance = balance.value.uiAmount;
        console.log("🚀 ~ it ~ Balance:", balance)

      } catch {
        // Token account not yet initiated has 0 balance
        initialBalance = 0;
      } 
  }); 
});
