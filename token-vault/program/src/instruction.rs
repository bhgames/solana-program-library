use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct InitVaultArgs {
    pub allow_further_share_creation: bool,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct AddTokenToInactiveVaultArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct ActivateVaultArgs {
    pub number_of_shares: u64,
}

/// Instructions supported by the Fraction program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum VaultInstruction {
    /// Initialize a token vault, starts inactivate. Add tokens in subsequent instructions, then activate.
    ///   0. `[writable]` Initialized fractional share mint with 0 tokens in supply
    ///   1. `[writable]` Initialized redeem treasury token account with 0 tokens in supply
    ///   2. `[writable]` Initialized fraction treasury token account with 0 tokens in supply
    ///   3. `[writable]` Uninitialized fractionalized token ledger account
    ///   4. `[]` Authority
    ///   5. `[]` Pricing Lookup Address
    ///   6. `[]` Token program
    ///   7. `[]` Rent sysvar
    InitVault(InitVaultArgs),

    /// Add a token to a inactive token vault
    ///   0. `[writable]` Uninitialized Token Fractional Registry account address (will be created and allocated by this endpoint)
    ///                   Address should be pda with seed of [PREFIX, fractional_token_ledger_address, token_mint_address]
    ///   1. `[writable]` Initialized Token account
    ///   2. `[writable]` Initialized Token safety deposit box account with authority of this program
    ///   3. `[writable]` Initialized inactive fractionalized token vault
    ///   4. `[signer]` Payer
    ///   5. `[]` Transfer Authority to move desired token amount from token account to safety deposit
    ///   6. `[]` Token program
    ///   7. `[]` Rent sysvar
    ///   8. `[]` System account sysvar
    AddTokenToInactiveVault(AddTokenToInactiveVaultArgs),

    ///   0. `[writable]` Initialized inactivated fractionalized token vault
    ///   1. `[writable]` Fraction mint
    ///   2. `[writable]` Fraction treasury
    ///   3. `[]` Fraction mint authority for the program
    ///   4. `[]` Token program
    ActivateVault(ActivateVaultArgs),

    ///   0. `[writable]` Initialized activated token vault
    ///   1. `[writable]` Token account containing your portion of the outstanding fraction shares
    ///   2. `[writable]` Token account of the redeem_treasury mint type that you will pay with
    ///   3. `[writable]` Fraction mint
    ///   4. `[writable]` Fraction treasury account
    ///   5. `[writable]` Redeem treasury account
    ///   6. `[]` Transfer authority for the  token account that you will pay with
    ///   7. `[]` Burn authority for the fraction token account containing your outstanding fraction shares
    ///   8. `[]` PDA-based Burn authority for the fraction treasury account containing the uncirculated shares
    ///   9. `[]` External pricing lookup address
    ///   10. `[]` Token program
    CombineVault,

    ///   0. `[writable]` Initialized Token account containing your fractional shares
    ///   1. `[writable]` Initialized Destination token account where you wish your proceeds to arrive
    ///   1. `[writable]` Fraction mint
    ///   1. `[writable]` Redeem treasury account
    ///   2. `[]` Transfer authority for the transfer of proceeds from redeem treasury to destination
    ///   3. `[]` Burn authority for the burning of all your fractional shares
    ///   4. `[]`  Combined token vault
    ///   5. `[]` Token program
    ///   6. `[]` Rent sysvar
    RedeemShares,
}
/*
/// Creates an CreateFractionAccounts instruction
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts(
    program_id: Pubkey,
    name_symbol_account: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    allow_duplication: bool,
    update_authority_is_signer: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(name_symbol_account, false),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: FractionInstruction::CreateFractionAccounts(CreateFractionAccountArgs {
            data: Data { name, symbol, uri },
            allow_duplication,
        })
        .try_to_vec()
        .unwrap(),
    }
}
*/
