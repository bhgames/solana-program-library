//! Program state processor
use crate::{
    error::TimelockError,
    state::governance::Governance,
    state::{
        custom_single_signer_transaction::{CustomSingleSignerTransaction, MAX_ACCOUNTS_ALLOWED},
        enums::ProposalStateStatus,
        governance::TIMELOCK_CONFIG_LEN,
        proposal::Proposal,
        proposal_state::ProposalState,
    },
    utils::{assert_account_equiv, assert_executing, assert_initialized, execute, ExecuteParams},
    PROGRAM_AUTHORITY_SEED,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    instruction::Instruction,
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

/// Execute an instruction
pub fn process_execute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    number_of_extra_accounts: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let transaction_account_info = next_account_info(account_info_iter)?;
    let timelock_state_account_info = next_account_info(account_info_iter)?;
    let program_to_invoke_info = next_account_info(account_info_iter)?;
    let proposal_account_info = next_account_info(account_info_iter)?;
    let governance_account_info = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;

    let mut timelock_state: ProposalState = assert_initialized(timelock_state_account_info)?;
    let proposal: Proposal = assert_initialized(proposal_account_info)?;
    let governance: Governance = assert_initialized(governance_account_info)?;
    let clock = &Clock::from_account_info(clock_info)?;
    // For now we assume all transactions are CustomSingleSignerTransactions even though
    // this will not always be the case...we need to solve that inheritance issue later.
    let mut transaction: CustomSingleSignerTransaction =
        assert_initialized(transaction_account_info)?;

    let time_elapsed = match clock.slot.checked_sub(timelock_state.voting_ended_at) {
        Some(val) => val,
        None => return Err(TimelockError::NumericalOverflow.into()),
    };

    if time_elapsed < transaction.slot {
        return Err(TimelockError::TooEarlyToExecute.into());
    }

    assert_account_equiv(timelock_state_account_info, &proposal.state)?;
    assert_account_equiv(governance_account_info, &proposal.config)?;

    let council_mint_seed = governance
        .council_mint
        .as_ref()
        .map(|key| key.as_ref())
        .unwrap_or(&[]);

    let mut seeds = vec![
        PROGRAM_AUTHORITY_SEED,
        program_id.as_ref(),
        governance.governance_mint.as_ref(),
        council_mint_seed,
        governance.program.as_ref(),
    ];

    let (governance_authority, bump_seed) = Pubkey::find_program_address(&seeds[..], program_id);
    let mut account_infos: Vec<AccountInfo> = vec![];
    if number_of_extra_accounts > (MAX_ACCOUNTS_ALLOWED - 2) as u8 {
        return Err(TimelockError::TooManyAccountsInInstruction.into());
    }
    let mut added_authority = false;

    for _ in 0..number_of_extra_accounts {
        let next_account = next_account_info(account_info_iter)?.clone();
        if next_account.data_len() == TIMELOCK_CONFIG_LEN {
            // You better be initialized, and if you are, you better at least be mine...
            let _nefarious_config: Governance = assert_initialized(&next_account)?;
            assert_account_equiv(&next_account, &proposal.config)?;
            added_authority = true;

            if next_account.key != &governance_authority {
                return Err(TimelockError::InvalidGovernanceKey.into());
            }
        }
        account_infos.push(next_account);
    }

    account_infos.push(program_to_invoke_info.clone());

    if !added_authority {
        if governance_account_info.key != &governance_authority {
            return Err(TimelockError::InvalidGovernanceKey.into());
        }
        account_infos.push(governance_account_info.clone());
    }

    assert_executing(&timelock_state)?;

    if transaction.executed == 1 {
        return Err(TimelockError::TimelockTransactionAlreadyExecuted.into());
    }

    let message: Message = match bincode::deserialize::<Message>(
        &transaction.instruction[0..transaction.instruction_end_index as usize + 1],
    ) {
        Ok(val) => val,
        Err(_) => return Err(TimelockError::InstructionUnpackError.into()),
    };
    let serialized_instructions = message.serialize_instructions();
    let instruction: Instruction =
        match Message::deserialize_instruction(0, &serialized_instructions) {
            Ok(val) => val,
            Err(_) => return Err(TimelockError::InstructionUnpackError.into()),
        };

    let bump = &[bump_seed];
    seeds.push(bump);
    let authority_signer_seeds = &seeds[..];

    execute(ExecuteParams {
        instruction,
        authority_signer_seeds,
        account_infos,
    })?;

    transaction.executed = 1;

    CustomSingleSignerTransaction::pack(
        transaction,
        &mut transaction_account_info.data.borrow_mut(),
    )?;

    timelock_state.number_of_executed_transactions = match timelock_state
        .number_of_executed_transactions
        .checked_add(1)
    {
        Some(val) => val,
        None => return Err(TimelockError::NumericalOverflow.into()),
    };

    if timelock_state.number_of_executed_transactions == timelock_state.number_of_transactions {
        timelock_state.status = ProposalStateStatus::Completed
    }

    ProposalState::pack(
        timelock_state,
        &mut timelock_state_account_info.data.borrow_mut(),
    )?;
    Ok(())
}
