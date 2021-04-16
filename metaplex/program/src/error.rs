//! Error types

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the Metaplex program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MetaplexError {
    /// Invalid instruction data passed in.
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,

    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    /// Already initialized
    #[error("Already initialized")]
    AlreadyInitialized,

    /// Uninitialized
    #[error("Uninitialized")]
    Uninitialized,

    /// Account does not have correct owner
    #[error("Account does not have correct owner")]
    IncorrectOwner,

    /// NumericalOverflowError
    #[error("NumericalOverflowError")]
    NumericalOverflowError,

    /// Token transfer failed
    #[error("Token transfer failed")]
    TokenTransferFailed,
    /// Token mint to failed
    #[error("Token mint to failed")]
    TokenMintToFailed,
    /// Token burn failed
    #[error("Token burn failed")]
    TokenBurnFailed,

    /// Invalid program authority provided
    #[error("Invalid program authority provided")]
    InvalidAuthority,

    /// Vault's authority does not match the expected pda with seed ['metaplex', auction_key]
    #[error("Vault's authority does not match the expected ['metaplex', auction_key]")]
    VaultAuthorityMismatch,

    /// Auction's authority does not match the expected pda with seed ['metaplex', auction_key]
    #[error(
        "Auction's authority does not match the expected pda with seed ['metaplex', auction_key]"
    )]
    AuctionAuthorityMismatch,

    /// The authority passed to the call does not match the authority on the auction manager!
    #[error(
        "The authority passed to the call does not match the authority on the auction manager!"
    )]
    AuctionManagerAuthorityMismatch,

    /// Auction Manager does not have the appropriate pda key with seed ['metaplex', auction_key]
    #[error(
        "Auction Manager does not have the appropriate pda key with seed ['metaplex', auction_key]"
    )]
    AuctionManagerKeyMismatch,

    /// External Price Account Owner must be this program
    #[error("External Price Account Owner must be this program")]
    ExternalPriceAccountOwnerMismatch,

    /// Vault's external pricing account needs to match the external pricing account given
    #[error("Vault's external pricing account needs to match the external pricing account given")]
    VaultExternalPricingMismatch,

    /// Auction is not auctioning off the vault given!
    #[error("Auction is not auctioning off the vault given!")]
    AuctionVaultMismatch,

    /// Vault given does not match that on given auction manager!
    #[error("Vault given does not match that on given auction manager!")]
    AuctionManagerVaultMismatch,

    /// The safety deposit box given does not belong to the given vault!
    #[error("The safety deposit box given does not belong to the given vault!")]
    SafetyDepositBoxVaultMismatch,

    /// The store given does not belong to the safety deposit box given!
    #[error("The store given does not belong to the safety deposit box given!")]
    SafetyDepositBoxStoreMismatch,

    /// The metadata given does not match the mint on the safety deposit box given!
    #[error("The metadata given does not match the mint on the safety deposit box given!")]
    SafetyDepositBoxMetadataMismatch,

    /// The mint given does not match the mint on the given safety deposit box!
    #[error("The mint given does not match the mint on the given safety deposit box!")]
    SafetyDepositBoxMintMismatch,

    /// The mint is owned by a different token program than the one used by this auction manager!
    #[error(
        "The mint is owned by a different token program than the one used by this auction manager!"
    )]
    TokenProgramMismatch,

    /// Only active vaults may be used in auction managers!
    #[error("Only active vaults may be used in auction managers!")]
    VaultNotActive,

    /// Cannot auction off an empty vault!
    #[error("Cannot auction off an empty vault!")]
    VaultCannotEmpty,

    /// Listed a safety deposit box index that does not exist in this vault
    #[error("Listed a safety deposit box index that does not exist in this vault")]
    InvalidSafetyDepositBox,

    /// Cant use a limited supply edition for an open edition as you may run out of editions to print
    #[error("Cant use a limited supply edition for an open edition as you may run out of editions to print")]
    CantUseLimitedSupplyEditionsWithOpenEditionAuction,

    /// This safety deposit box is not listed as a prize in this auction manager!
    #[error("This safety deposit box is not listed as a prize in this auction manager!")]
    SafetyDepositBoxNotUsedInAuction,

    /// Auction Manager Authority needs to be signer for this action!
    #[error("Auction Manager Authority needs to be signer for this action!")]
    AuctionManagerAuthorityIsNotSigner,

    /// Either you have given a non-existent edition address or you have given the address to a different token-metadata program than was used to make this edition!
    #[error("Either you have given a non-existent edition address or you have given the address to a different token-metadata program than was used to make this edition!")]
    InvalidEditionAddress,

    /// There are not enough editions available for this auction!
    #[error("There are not enough editions available for this auction!")]
    NotEnoughEditionsAvailableForAuction,

    /// The store in the safety deposit is empty, so you have nothing to auction!
    #[error("The store in the safety deposit is empty, so you have nothing to auction!")]
    StoreIsEmpty,

    /// Cannot auction off more than one of the master edition itself!
    #[error("Cannot auction off more than one of the master edition itself!")]
    CannotAuctionOffMoreThanOneOfMasterEditionItself,

    /// Cannot auction off more than one of a limited edition!
    #[error("Cannot auction off more than one of a limited edition!")]
    CannotAuctionOffMoreThanOneOfLimitedEdition,

    /// Not enough tokens to supply winners!
    #[error("Not enough tokens to supply winners!")]
    NotEnoughTokensToSupplyWinners,
}

impl PrintProgramError for MetaplexError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MetaplexError> for ProgramError {
    fn from(e: MetaplexError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MetaplexError {
    fn type_of() -> &'static str {
        "Metaplex Error"
    }
}
