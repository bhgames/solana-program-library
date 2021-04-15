#![deny(missing_docs)]
//! A timelock program for the Solana blockchain.

pub mod entrypoint;
pub mod error;
/// instruction
pub mod instruction;
/// processor
pub mod processor;
///state
pub mod state;
/// utils
pub mod utils;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("NFTMetadata11111111111111111111111111111111");
