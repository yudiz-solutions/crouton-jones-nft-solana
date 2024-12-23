use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    },
};

#[derive(Accounts)]
pub struct InitToken<'info> {
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // UncheckedAccount type to indicate that another program will create this account.
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer, // Wallet address
    )]
    pub mint: Account<'info, Mint>, // Token mint account

    #[account(
        mut,
        seeds = [b"token_holder"],
        bump
    )]
    pub token_holder: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint, 
        associated_token::authority = token_holder,
    )]
    pub destination: Account<'info, TokenAccount>, // TOKEN ATA address

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}  
