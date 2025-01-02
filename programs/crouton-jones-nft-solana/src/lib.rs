use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::token::{initialize_mint, mint_to, InitializeMint, MintTo};
use anchor_spl::associated_token::{create, Create};
use mpl_token_metadata::types::{Collection, DataV2};
use mpl_token_metadata::instructions::{
    CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
    CreateMetadataAccountV3InstructionArgs, PrintV2, PrintV2InstructionArgs
};

pub mod utils;
mod structures;
use structures::*;

use solana_program::program::invoke;

declare_id!("CeyTr5Ums5UzGynXdhVm5iZsqipouk2GJa8urRKCzwgC");

#[program]
pub mod crouton_jones_nft_solana {

    use super::*;
    pub fn mint(
        ctx: Context<MintNft>,
        quantity: u64,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let uri = metadata_uri.clone();
        
        msg!("Minting NFTs");
        create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;
    
        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;
    
        create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
    
        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            quantity,
        )?;
    
        let data_v2 = DataV2 {
            name: metadata_title,
            symbol: metadata_symbol,
            uri: metadata_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        let args = CreateMetadataAccountV3InstructionArgs {
            data: data_v2,
            is_mutable: true,
            collection_details: None,
        };
        let create_ma = CreateMetadataAccountV3 {
            metadata: ctx.accounts.metadata.key(),
            mint: ctx.accounts.mint.key(),
            mint_authority: ctx.accounts.mint_authority.key(),
            payer: ctx.accounts.mint_authority.key(),
            update_authority: (ctx.accounts.mint_authority.key(), true),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        };
        let create_ma = create_ma.instruction(args);
        let accounts = &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            // ctx.accounts.collection.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        invoke(&create_ma, accounts)?;
    
        if quantity == 1 {
            let args = CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            };
            let create_mea = CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition.key(),
                mint: ctx.accounts.mint.key(),
                update_authority: ctx.accounts.mint_authority.key(),
                mint_authority: ctx.accounts.mint_authority.key(),
                payer: ctx.accounts.mint_authority.key(),
                metadata: ctx.accounts.metadata.key(),
                token_program: ctx.accounts.token_program.key(),
                system_program: ctx.accounts.system_program.key(),
                rent: Some(ctx.accounts.rent.key()),
            };
            let create_mea = create_mea.instruction(args);
            let accounts = &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ];
            invoke(&create_mea, accounts)?;
        }
        else{
            
        }
    
        // emit!(MintEvent {
        //     uri: uri,
        //     mint_key: ctx.accounts.mint.key(),
        //     owner: ctx.accounts.mint_authority.key(),
        //     collection: ctx.accounts.collection.key(),
        //     quantity: quantity
        // });
    
        Ok(())
    }    
 
   pub fn mint_edition(
    ctx: Context<MintEdition>,
    edition_number: u64,
) -> Result<()> {
    msg!("Minting Edition #{}", edition_number);

    // Create new mint account
    create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.edition_mint.to_account_info(),
            },
        ),
        10000000,
        82,
        &ctx.accounts.token_program.key(),
    )?;

    // Initialize new mint
    initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.edition_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        &ctx.accounts.payer.key(),
        Some(&ctx.accounts.payer.key()),
    )?;

    // Create token account for new edition
    create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.edition_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.edition_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    ))?;

    // Mint one token
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.edition_mint.to_account_info(),
                to: ctx.accounts.edition_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        1,
    )?;
msg!("Creating metadata account");
    // Create the print edition instruction
    let print_edition_ix = PrintV2 {
        edition: ctx.accounts.edition.key(),
        edition_metadata: ctx.accounts.edition_metadata.key(),
        edition_mint: (ctx.accounts.edition_mint.key(), false),
        edition_token_account_owner: ctx.accounts.payer.key(),
        edition_token_account: ctx.accounts.edition_token_account.key(),
        edition_mint_authority: ctx.accounts.payer.key(),
        edition_token_record: None,
        edition_marker_pda: ctx.accounts.edition.key(),
        master_token_account_owner: (ctx.accounts.payer.key(), false),
        master_token_account: ctx.accounts.master_token_account.key(),
        master_edition: ctx.accounts.master_edition.key(),
        master_metadata: ctx.accounts.master_metadata.key(),
        system_program: ctx.accounts.system_program.key(),
        update_authority: ctx.accounts.payer.key(),
        payer: ctx.accounts.payer.key(),
        spl_token_program: ctx.accounts.token_program.key(),
        spl_ata_program: ctx.accounts.associated_token_program.key(),
        sysvar_instructions: ctx.accounts.system_program.key(),
        holder_delegate_record: None,
        delegate: None,
    }.instruction(PrintV2InstructionArgs {
        edition_number  // Changed from edition to edition_number
    });
    msg!("Calling print edition instruction");
    
    solana_program::program::invoke(
        &print_edition_ix,
        &[
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.edition_metadata.to_account_info(),
            ctx.accounts.edition_mint.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.master_metadata.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
        ],
    )?;

    Ok(())
}

}

