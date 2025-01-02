use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
};
#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: will be created with anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: will be used
    // #[account(mut)]
    // pub collection: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: will be created with Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintEdition<'info> {
    #[account(mut)]
    pub edition_mint: Signer<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub edition_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub edition_metadata: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub edition: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_mint: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_token_account: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_metadata: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub edition_marker_pda: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the Token Metadata Program
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the instructions sysvar
    pub sysvar_instructions: UncheckedAccount<'info>,
}