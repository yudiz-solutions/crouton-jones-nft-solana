use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
    },
    token::{self, mint_to, MintTo},
};
pub mod utils;
mod structures;
use structures::*;

declare_id!("3Bj1KmAeobZ2rBg1GiitVXxoV6Z3zQZtCtYHTgCbUrcj");

#[program]
pub mod crouton_jones_nft_solana {
    use super::*;
    use utils::mint_burn::{mint_token_utility};
    pub fn init_token(
        ctx: Context<InitToken>,
        name: String,
        symbol: String,
        uri: String,
        quantity: u64,
    ) -> Result<()> {
        msg!("Creating init token mint...");
        let token_data: DataV2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.mint.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

        msg!("Token mint created successfully.");

        msg!("Minting tokens...");

        // mint_to(
        //     CpiContext::new(
        //         ctx.accounts.token_program.to_account_info(),
        //         MintTo {
        //             authority: ctx.accounts.payer.to_account_info(),
        //             to: ctx.accounts.destination.to_account_info(),
        //             mint: ctx.accounts.mint.to_account_info(),
        //         }
        //         // &signer,
        //     ),
        //     quantity,
        // )?;
        mint_token_utility(
            quantity,
            &ctx.accounts.payer,
            &ctx.accounts.destination,
            &ctx.accounts.mint,
            &ctx.accounts.token_program,
        )?;
        msg!("Token minted successfully.");

        Ok(())
    }
}

