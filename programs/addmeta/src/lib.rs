use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use solana_program::program::invoke_signed;
use std::convert::Into;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
   };

use mpl_token_metadata::{instruction::create_metadata_accounts_v2};

pub const CTRLSEED: &[u8] = b"CTRLv1";

declare_id!("3WEXkDmTYHfGjLWnFW2NoAhgEcDgcPn2LHKBPGLUcgq2");

#[program]
mod addmeta {
    use super::*;

    pub fn mint_ctrl(ctx: Context<MintCtrl>, bump: u8, amount: u64) -> Result<()> {
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&[CTRLSEED, &[bump]]],
            ),amount,
        )?;
        Ok(())
    }    

    pub fn tok_meta(ctx: Context<TokMeta>, bump: u8) -> Result<()> {
       let ix = create_metadata_accounts_v2(
            *ctx.accounts.metadata_program.to_account_info().key, // program_id,
            *ctx.accounts.metadata_pda.to_account_info().key, // metadata_account,
            *ctx.accounts.mint.to_account_info().key, //mint,
            *ctx.accounts.mint.to_account_info().key, //mint_authority,
            *ctx.accounts.payer.to_account_info().key, //payer,
            *ctx.accounts.updauth.to_account_info().key, //update_authority,
            String::from("CTRL - Program Controlled Token"), // name,
            String::from("CTRL"), // symbol,
            String::from("https://bernieblume.github.io/Ctrl/Ctrl.json"), // uri,
            None, // creators,
            0u16, //seller_fee_basis_points,
            false, // update_authority_is_signer,
            true, // is_mutable,
            None, // collection,
            None, // uses,
            // for create_metadata_accounts_v3, add:     None, // collection_details
        );
        invoke_signed(
            &ix,
            &[
                ctx.accounts.metadata_program.to_account_info().clone(), // Metadata program id
                ctx.accounts.metadata_pda.to_account_info().clone(), // Metadata account
                ctx.accounts.mint.to_account_info().clone(), // Mint
                ctx.accounts.mint.to_account_info().clone(), // Mint Authority
                ctx.accounts.payer.to_account_info().clone(), // Payer
                ctx.accounts.updauth.to_account_info().clone(), // Update Authority
                ctx.accounts.system_program.to_account_info().clone(), // System Program
                ctx.accounts.rent.to_account_info().clone(), // Rent Sysvar
            ],
            &[
                &[CTRLSEED.as_ref(), &[bump]],
            ],
        )?;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(okubump: u8, amount: u64)]
pub struct MintCtrl<'info> {
    #[account(init_if_needed, payer = payer, seeds = [CTRLSEED.as_ref()], bump, mint::decimals = 6, mint::authority = mint)]
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = payer, associated_token::mint = mint, associated_token::authority = payer)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct TokMeta<'info> {
    #[account(seeds = [CTRLSEED.as_ref()], bump = bump, mint::decimals = 6, mint::authority = mint)]
    pub mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because it will be checked in the inner instruction
    pub mintauth: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because it will be checked in the inner instruction
    pub updauth: AccountInfo<'info>,
    /// CHECK: This is not dangerous because it's being checked by the inner instruction
    #[account(mut)]
    pub metadata_pda: AccountInfo <'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because it will be checked in the inner instruction
    pub metadata_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}