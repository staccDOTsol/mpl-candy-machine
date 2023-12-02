use std::collections::BTreeMap;

use anchor_lang::{prelude::*, solana_program::sysvar};

use mpl_candy_machine_core::{AccountVersion, CandyMachine, RequestAccountData};
use switchboard_solana::{AttestationProgramState, AttestationQueueAccountData, FunctionAccountData, FunctionRequestInitAndTrigger, SWITCHBOARD_ATTESTATION_PROGRAM_ID, Mint as m2, AssociatedToken};

use crate::{
    guards::{CandyGuardError, EvaluationContext},
    state::{CandyGuard, SEED},
};

use super::{mint_v2::process_mint, MintAccounts, Token};

pub fn mint<'info>(
    ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
    mint_args: Vec<u8>,
    label: Option<String>,
) -> Result<()> {
    msg!("(Deprecated as of 1.0.0) Use MintV2 instead");

    if !matches!(ctx.accounts.candy_machine.version, AccountVersion::V1) {
        return err!(CandyGuardError::InvalidAccountVersion);
    }

    let accounts = MintAccounts {
        candy_guard: &ctx.accounts.candy_guard,
        candy_machine: &ctx.accounts.candy_machine,
        candy_machine_authority_pda: ctx.accounts.candy_machine_authority_pda.to_account_info(),
        _candy_machine_program: ctx.accounts.candy_machine_program.to_account_info(),
        collection_delegate_record: ctx.accounts.collection_authority_record.to_account_info(),
        collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
        collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
        collection_mint: ctx.accounts.collection_mint.to_account_info(),
        collection_update_authority: ctx.accounts.collection_update_authority.to_account_info(),
        nft_master_edition: ctx.accounts.nft_master_edition.to_account_info(),
        nft_metadata: ctx.accounts.nft_metadata.to_account_info(),
        nft_mint: ctx.accounts.nft_mint.to_account_info(),
        nft_mint_authority: ctx.accounts.nft_mint_authority.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        spl_ata_program: None,
        spl_token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        sysvar_instructions: ctx.accounts.instruction_sysvar_account.to_account_info(),
        token: None,
        token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
        token_record: None,
        minter: ctx.accounts.payer.to_account_info(),
        remaining: ctx.remaining_accounts,
        authorization_rules_program: None,
        authorization_rules: None,
        req: ctx.accounts.req.to_account_info(),
        switchboard: ctx.accounts.switchboard.to_account_info(),
        switchboard_state: ctx.accounts.switchboard_state.to_account_info(),
        switchboard_attestation_queue: ctx.accounts.switchboard_attestation_queue.to_account_info(),
        switchboard_function: ctx.accounts.switchboard_function.to_account_info(),
        switchboard_request: ctx.accounts.switchboard_request.to_account_info(),
        switchboard_request_escrow: ctx.accounts.switchboard_request_escrow.to_account_info(),
        switchboard_mint: ctx.accounts.switchboard_mint.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),

    };

    // evaluation context for this transaction
    let mut ctx = EvaluationContext {
        accounts,
        account_cursor: 0,
        args_cursor: 0,
        indices: BTreeMap::new(),
    };

    process_mint(&mut ctx, mint_args, label)
}

#[derive(Accounts)]
#[rustfmt::skip]
pub struct Mint<'info> {
    #[account(seeds = [SEED, candy_guard.base.key().as_ref()], bump = candy_guard.bump)]
    pub candy_guard: Account<'info, CandyGuard>,

    /// CHECK: account constraints checked in account trait
    #[account(address = mpl_candy_machine_core::id())]
    pub candy_machine_program: AccountInfo<'info>,

    #[account(mut,constraint = candy_guard.key() == candy_machine.mint_authority)]
    pub candy_machine: Box<Account<'info, CandyMachine>>,

    // seeds and bump are not validated by the candy guard, they will be validated
    // by the CPI'd candy machine mint instruction
    /// CHECK: account constraints checked in account trait
    #[account(mut)]
    pub candy_machine_authority_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // with the following accounts we aren't using anchor macros because they are CPI'd
    // through to token-metadata which will do all the validations we need on them.
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,
    
    pub nft_mint_authority: Signer<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    pub nft_master_edition: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    pub collection_authority_record: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    pub collection_mint: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    pub collection_master_edition: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    pub collection_update_authority: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    
    pub system_program: Program<'info, System>,

    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::id())]
    pub instruction_sysvar_account: UncheckedAccount<'info>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<RequestAccountData>(),
        seeds = [&authority.key().as_ref()],
        payer = payer,
        bump,
    )]
    pub req: AccountLoader<'info, RequestAccountData>,

    /// CHECK:
    pub authority: AccountInfo<'info>,

    /// CHECK: Switchboard attestation program
    #[account(executable, address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    pub switchboard: AccountInfo<'info>,

    pub switchboard_state: AccountLoader<'info, AttestationProgramState>,
    pub switchboard_attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,
    #[account(mut)]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    /// CHECK: validated by Switchboard CPI
    #[account(
        mut,
        signer,
        owner = system_program.key(),
        constraint = switchboard_request.lamports() == 0
      )]
    pub switchboard_request: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut,
        owner = system_program.key(),
        constraint = switchboard_request_escrow.lamports() == 0
      )]
    pub switchboard_request_escrow: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub switchboard_mint: Account<'info, m2>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
