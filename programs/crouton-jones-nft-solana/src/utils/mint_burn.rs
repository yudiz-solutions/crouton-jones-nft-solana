use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount,Mint, MintTo, mint_to};

pub fn mint_token_utility<'info>(
    quantity: u64,
    payer: &Signer<'info>,
    destination: &Account<'info, TokenAccount>,
    mint: &Account<'info, Mint>,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                authority: payer.to_account_info(),
                to: destination.to_account_info(),
                mint: mint.to_account_info(),
            }
            // &signer,
        ),
        quantity,
    )
}