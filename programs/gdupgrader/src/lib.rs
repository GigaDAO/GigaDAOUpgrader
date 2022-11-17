use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::{bpf_loader_upgradeable, sysvar};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::loader_upgradeable_instruction::UpgradeableLoaderInstruction;
use solana_program::program::invoke_signed;
use std::str::FromStr;

use anchor_lang::solana_program::bpf_loader_upgradeable::upgrade as program_upgrade; // TODO use this instead for building...

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// consts
pub const MIN_ACCOUNT_LEN: usize = 9;
const MULTISIG_PDA_SEED: &[u8] = b"multisig_pda_seed";

#[program]
pub mod gdupgrader {
    use super::*;

    pub fn upgrade(ctx: Context<Upgrade>) -> Result<()> {

        // create signer seed
        let (multisig_pda, bump_seed) = Pubkey::find_program_address(&[MULTISIG_PDA_SEED], ctx.program_id);
        if multisig_pda != ctx.accounts.multisig_pda.key() {
            return err!(ErrorCode::InvalidAuthPda);
        }
        let seeds = &[&MULTISIG_PDA_SEED[..], &[bump_seed]];
        let signer = &[&seeds[..]];

        let instruction = Instruction::new_with_bincode(
            bpf_loader_upgradeable::id(),
            &UpgradeableLoaderInstruction::Upgrade,
            vec![
                AccountMeta::new(ctx.accounts.target_program_buffer.key(), false), // target program buffer
                AccountMeta::new(ctx.accounts.target_program.key(), false), // target program
                AccountMeta::new(ctx.accounts.source_buffer.key(), false), // tmp buffer account
                AccountMeta::new(ctx.accounts.signer.key(), false), // spill account
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(sysvar::clock::id(), false),
                AccountMeta::new_readonly(ctx.accounts.multisig_pda.key(), true), // multisig PDA
            ],
        );

        let accounts = [
            ctx.accounts.target_program_buffer.to_account_info().clone(),
            ctx.accounts.target_program.to_account_info().clone(),
            ctx.accounts.source_buffer.to_account_info().clone(),
            ctx.accounts.signer.to_account_info().clone(),
            ctx.accounts.rent.to_account_info().clone(),
            ctx.accounts.clock.to_account_info().clone(),
            ctx.accounts.multisig_pda.to_account_info().clone(),
        ];

        invoke_signed(
            &instruction,
            &accounts,
            signer,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut)]
    /// CHECK: bypass
    pub target_program_buffer: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: bypass
    pub target_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: bypass
    pub source_buffer: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    #[account(
    init_if_needed,
    seeds = [MULTISIG_PDA_SEED],
    bump,
    payer = signer,
    space = MIN_ACCOUNT_LEN,
    )]
    pub multisig_pda: Account<'info, AuthAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

// custom errors
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Authorizer PDA.")]
    InvalidAuthPda,
}




