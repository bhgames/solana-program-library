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

    /// Only active vaults may be used in auction managers!
    #[error("Only active vaults may be used in auction managers!")]
    VaultNotActive,

    /// Cannot auction off an empty vault!
    #[error("Cannot auction off an empty vault!")]
    VaultCannotEmpty,
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
