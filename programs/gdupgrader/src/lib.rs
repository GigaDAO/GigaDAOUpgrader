use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::{bpf_loader_upgradeable, sysvar};
use anchor_lang::solana_program::bpf_loader_upgradeable::UpgradeableLoaderState;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::loader_upgradeable_instruction::UpgradeableLoaderInstruction;
use solana_program::program::invoke_signed;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("7zytPdaZiXNjYQh1cStfAcFws7ZRhSLtUfhdoev9vp5G");

// consts
pub const MIN_ACCOUNT_LEN: usize = 9;
const MULTISIG_PDA_SEED: &[u8] = b"multisig_pda_seed";
const GIGS_VAULT_PDA_SEED: &[u8] = b"gigs_vault_pda_seed";
const PROPOSAL_PDA_SEED: &[u8] = b"proposal_pda_seed";

// TODO remove this to initialize params
const APPROVAL_THRESHOLD: u64 = 1_100_000_000_000; // 110M GIGS * 4 decimals
const PROPOSAL_MINIMUM: u64 = 500_000_000_000; // 50M GIGS * 4 decimals

#[program]
pub mod gdupgrader {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        governance_token_mint: Pubkey,
        approval_threshold: u64,
        proposal_minimum: u64,
    ) -> Result<()> {
        ctx.accounts.proposal.is_active = false;
        ctx.accounts.proposal.proposal_id = 0;
        ctx.accounts.proposal.governance_token_mint = governance_token_mint;
        ctx.accounts.proposal.approval_threshold = approval_threshold;
        ctx.accounts.proposal.proposal_minimum = proposal_minimum;
        Ok(())
    }
    pub fn propose(
        ctx: Context<Propose>,
        proposal_type: ProposalType,
        target_buffer: Pubkey,
        source_buffer: Pubkey,
        new_authority: Pubkey,
        amount: u64,
    ) -> Result<()> {

        // check amount >= minimum proposal amount
        if amount < ctx.accounts.proposal.proposal_minimum {
            return err!(ErrorCode::InsufficientAmount);
        }

        // transfer amount
        let signer_handle = &ctx.accounts.signer;
        let tx_handle = ctx.accounts.sender_gigs_ata.to_account_info();
        let rx_handle = ctx.accounts.gigs_vault.to_account_info();
        let token_program_acct_info = ctx.accounts.token_program.to_account_info();
        transfer_tokens(signer_handle, tx_handle, rx_handle, token_program_acct_info, amount)?;

        // init proposal
        ctx.accounts.proposal.proposal_id += 1;
        ctx.accounts.proposal.proposal_type = proposal_type;
        ctx.accounts.proposal.target_buffer = target_buffer;
        ctx.accounts.proposal.source_buffer = source_buffer;
        ctx.accounts.proposal.new_authority = new_authority;
        ctx.accounts.proposal.num_votes = amount;
        ctx.accounts.proposal.is_active = true;

        Ok(())
    }
    pub fn cast_ballot(ctx: Context<CastBallot>) -> Result<()> {
        // TODO check if proposal is active
        // TODO check proposal id matches
        // TODO initialize ballot
        // TODO update proposal approval
        // TODO transfer to vault
        Ok(())
    }
    pub fn close_ballot(ctx: Context<CloseBallot>) -> Result<()> {
        // TODO transfer gigs from vault to receiver
        // TODO if ballot matches active, reduce approval
        Ok(())
    }
    pub fn execute_set_authority(ctx: Context<ExecuteSetAuthority>) -> Result<()> {
        // TODO check proposal is of type: set_authority
        // TODO check program data account and new authority match proposal
        // TODO check approval has requisite threshold

        Ok(())
    }
    pub fn execute_upgrade_program(ctx: Context<ExecuteUpgradeProgram>) -> Result<()> {

        // TODO check proposal is of type: upgrade_program
        // TODO check source buffer and target program match proposal
        // TODO check approval has requisite threshold

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
                AccountMeta::new(ctx.accounts.signer.key(), false), // spill account (signer or other?)
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(sysvar::clock::id(), false),
                AccountMeta::new_readonly(ctx.accounts.multisig_pda.key(), true), // multisig PDA
            ],
        );

        let mut accounts = [
            ctx.accounts.target_program_buffer.to_account_info().clone(),
            ctx.accounts.target_program.to_account_info().clone(),
            ctx.accounts.source_buffer.to_account_info().clone(),
            ctx.accounts.signer.to_account_info().clone(), // spill
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
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    seeds = [MULTISIG_PDA_SEED],
    bump,
    payer = signer,
    space = MIN_ACCOUNT_LEN,
    )]
    pub multisig_pda: Account<'info, AuthAccount>,
    #[account(
    init,
    seeds = [PROPOSAL_PDA_SEED],
    bump,
    payer = signer,
    space = 666,
    )]
    pub proposal: Account<'info, Proposal>,
    pub gigs_mint: Account<'info, Mint>, // TODO add constraint for this
    #[account(
    init,
    token::mint = gigs_mint,
    token::authority = multisig_pda,
    seeds = [GIGS_VAULT_PDA_SEED],
    bump,
    payer = signer,
    )]
    pub gigs_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [PROPOSAL_PDA_SEED],
    bump,
    )]
    pub proposal: Account<'info, Proposal>,
    pub gigs_mint: Account<'info, Mint>,
    #[account(
    mut,
    seeds = [GIGS_VAULT_PDA_SEED],
    bump,
    )]
    pub gigs_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sender_gigs_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CastBallot<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseBallot<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [MULTISIG_PDA_SEED],
    bump,
    )]
    pub multisig_pda: Account<'info, AuthAccount>,
}

#[derive(Accounts)]
pub struct ExecuteSetAuthority<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteUpgradeProgram<'info> {
    #[account(mut)]
    pub target_program_buffer: Account<'info, ProgramData>,
    #[account(mut)]
    pub target_program: Account<'info, UpgradeableLoaderState>,
    #[account(mut)]
    pub source_buffer: Account<'info, UpgradeableLoaderState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    #[account(
    mut,
    seeds = [MULTISIG_PDA_SEED],
    bump,
    )]
    pub multisig_pda: Account<'info, AuthAccount>,
    pub system_program: Program<'info, System>,
    /// CHECK: bypass
    pub bpf_loader: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

#[account]
#[derive(Default)]
pub struct Proposal {

    // state
    pub is_active: bool,
    pub proposal_id: u64,
    pub num_votes: u64,
    pub proposal_type: ProposalType,

    // params
    pub target_buffer: Pubkey,
    pub source_buffer: Pubkey,
    pub new_authority: Pubkey,

    // config
    pub governance_token_mint: Pubkey,
    pub approval_threshold: u64,
    pub proposal_minimum: u64,

}

#[account]
#[derive(Default)]
pub struct Ballot {
    pub proposal_id: u64,
    pub num_votes: u64,
    pub voter_address: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalType {
    UpgradeProgram,
    SetAuthority,
}

impl Default for ProposalType {
    fn default() -> Self {
        ProposalType::UpgradeProgram
    }
}

// utils
pub fn transfer_tokens<'a>(
    signer: &Signer<'a>,
    tx_acct_info: AccountInfo<'a>,
    rx_acct_info: AccountInfo<'a>,
    token_program_info: AccountInfo<'a>,
    amount: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: tx_acct_info,
        to: rx_acct_info,
        authority: signer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program_info, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

// custom errors
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Proposal Amount.")]
    InsufficientAmount,
    #[msg("Invalid Authorizer PDA.")]
    InvalidAuthPda,
}




