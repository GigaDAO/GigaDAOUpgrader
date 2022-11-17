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




        // // TODO add accounts below to context
        //
        // // TODO replace with account info from fed in accounts
        // let bfp_program_upgrade_address = bpf_loader_upgradeable::id();
        // let buffer_address = &Pubkey::from_str("3rkWkQ1dzhVgdUSWqscBQqzBpB6nnzppbnnFaHPVuNwG")?;
        //
        // let upgrade_ix = program_upgrade(
        //     bfp_program_upgrade_address,
        //     buffer_address,
        //     pda_authority_address,
        //     spill_address,
        // );
        //
        // let accounts_iter = &mut accounts.iter();
        //
        // let signer_account = next_account_info(accounts_iter)?;
        // let multisig_account = next_account_info(accounts_iter)?;
        // let target_program = next_account_info(accounts_iter)?;
        // let program_buffer = next_account_info(accounts_iter)?;
        // let source_buffer = next_account_info(accounts_iter)?;
        // let rent_program = next_account_info(accounts_iter)?;
        // let clock_program = next_account_info(accounts_iter)?;
        //
        // let current_authority = multisig_account.key;
        //
        // let instruction = Instruction::new_with_bincode(
        //     bpf_loader_upgradeable::id(),
        //     &UpgradeableLoaderInstruction::Upgrade,
        //     vec![
        //         AccountMeta::new(*program_buffer.key, false),
        //         AccountMeta::new(*target_program.key, false),
        //         AccountMeta::new(*source_buffer.key, false),
        //         AccountMeta::new(*signer_account.key, false),
        //         AccountMeta::new_readonly(sysvar::rent::id(), false),
        //         AccountMeta::new_readonly(sysvar::clock::id(), false),
        //         AccountMeta::new_readonly(*current_authority, true),
        //     ],
        // );
        //
        // invoke_signed(
        //     &instruction,
        //     &[
        //         program_buffer.clone(),
        //         target_program.clone(),
        //         source_buffer.clone(),
        //         signer_account.clone(), // used as spill account
        //         rent_program.clone(),
        //         clock_program.clone(),
        //         multisig_account.clone(),
        //     ],
        //     &[&[b"multisig", target_program.key.as_ref(), &[multisig_bump]]],
        // )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init_if_needed,
    seeds = [MULTISIG_PDA_SEED],
    bump,
    payer = signer,
    space = MIN_ACCOUNT_LEN,
    )]
    pub multisig_pda: Account<'info, AuthAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}





