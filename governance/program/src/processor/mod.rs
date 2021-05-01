pub mod process_add_custom_single_signer_transaction;
pub mod process_add_signer;
pub mod process_create_empty_governance_voting_record;
pub mod process_create_governance;
pub mod process_delete_proposal;
pub mod process_deposit_source_tokens;
pub mod process_execute;
pub mod process_init_proposal;
pub mod process_remove_signer;
pub mod process_remove_transaction;
pub mod process_sign;
pub mod process_update_transaction_slot;
pub mod process_vote;
pub mod process_withdraw_voting_tokens;

use crate::instruction::GovernanceInstruction;
use process_add_custom_single_signer_transaction::process_add_custom_single_signer_transaction;
use process_add_signer::process_add_signer;
use process_create_empty_governance_voting_record::process_create_empty_governance_voting_record;
use process_create_governance::process_create_governance;
use process_delete_proposal::process_delete_proposal;
use process_deposit_source_tokens::process_deposit_source_tokens;
use process_execute::process_execute;
use process_init_proposal::process_init_proposal;
use process_remove_signer::process_remove_signer;
use process_remove_transaction::process_remove_transaction;
use process_sign::process_sign;
use process_update_transaction_slot::process_update_transaction_slot;
use process_vote::process_vote;
use process_withdraw_voting_tokens::process_withdraw_voting_tokens;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GovernanceInstruction::unpack(input)?;
    match instruction {
        GovernanceInstruction::InitProposal { name, desc_link } => {
            msg!("Instruction: Init Proposal");
            process_init_proposal(program_id, accounts, name, desc_link)
        }
        GovernanceInstruction::AddSignatory => {
            msg!("Instruction: Add Signer");
            process_add_signer(program_id, accounts)
        }
        GovernanceInstruction::RemoveSignatory => {
            msg!("Instruction: Remove Signer");
            process_remove_signer(program_id, accounts)
        }
        GovernanceInstruction::AddCustomSingleSignerTransaction {
            delay_slots,
            instruction,
            position,
            instruction_end_index,
        } => process_add_custom_single_signer_transaction(
            program_id,
            accounts,
            delay_slots,
            instruction,
            position,
            instruction_end_index,
        ),
        GovernanceInstruction::RemoveTransaction => {
            msg!("Instruction: Remove Transaction");
            process_remove_transaction(program_id, accounts)
        }
        GovernanceInstruction::UpdateTransactionDelaySlots { delay_slots } => {
            msg!("Instruction: Update Transaction Slot");
            process_update_transaction_slot(program_id, accounts, delay_slots)
        }
        GovernanceInstruction::DeleteProposal => {
            msg!("Instruction: Delete Proposal");
            process_delete_proposal(program_id, accounts)
        }
        GovernanceInstruction::SignProposal => {
            msg!("Instruction: Sign");
            process_sign(program_id, accounts)
        }
        GovernanceInstruction::Vote { vote } => {
            msg!("Instruction: Vote");
            process_vote(program_id, accounts, vote)
        }
        GovernanceInstruction::CreateGovernance {
            vote_threshold,
            minimum_slot_waiting_period,
            time_limit,
            name,
        } => {
            msg!("Instruction: Initialize Governance");
            process_create_governance(
                program_id,
                accounts,
                vote_threshold,
                minimum_slot_waiting_period,
                time_limit,
                name,
            )
        }
        GovernanceInstruction::Execute => {
            msg!("Instruction: Execute");
            process_execute(program_id, accounts)
        }
        GovernanceInstruction::DepositSourceTokens {
            voting_token_amount,
        } => {
            msg!("Instruction: Deposit Source Tokens");
            process_deposit_source_tokens(program_id, accounts, voting_token_amount)
        }
        GovernanceInstruction::WithdrawVotingTokens {
            voting_token_amount,
        } => {
            msg!("Instruction: Withdraw Voting Tokens");
            process_withdraw_voting_tokens(program_id, accounts, voting_token_amount)
        }

        GovernanceInstruction::CreateEmptyGovernanceVoteRecord => {
            msg!("Instruction: Create Empty Governance Voting Record");
            process_create_empty_governance_voting_record(program_id, accounts)
        }

        GovernanceInstruction::CreateGovAccount => {
            msg!("Instruction: CreateAccount");
            Ok(())
        }
    }
}
