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

        // let bfp_program_upgrade_address = bpf_loader_upgradeable::id();
        // let target_program_address = &Pubkey::from_str("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s")?;
        // let buffer_address = &Pubkey::from_str("3rkWkQ1dzhVgdUSWqscBQqzBpB6nnzppbnnFaHPVuNwG")?;
        //
        // let upgrade_ix = program_upgrade(
        //     bfp_program_upgrade_address,
        //     buffer_address,
        //     pda_authority_address,
        //     spill_address,
        // );

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

        //
        // invoke_signed(
        //     &instruction,
        //     &[
        //         program_buffer.clone(), // target program buffer
        //         target_program.clone(), // target program
        //         source_buffer.clone(), // tmp buffer account
        //         signer_account.clone(), // spill account (signer)
        //         rent_program.clone(),
        //         clock_program.clone(),
        //         multisig_account.clone(), // multisig PDA
        //     ],
        //     &[&[b"multisig", target_program.key.as_ref(), &[multisig_bump]]],
        // )?;

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




