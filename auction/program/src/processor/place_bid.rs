//! Places a bid on a running auction, the logic here implements a standard English auction
//! mechanism, once the auction starts, new bids can be made until 10 minutes has passed with no
//! new bid. At this point the auction ends.
//!
//! Possible Attacks to Consider:
//!
//! 1) A user bids many many small bids to fill up the buffer, so that his max bid wins.
//! 2) A user bids a large amount repeatedly to indefinitely delay the auction finishing.
//!
//! A few solutions come to mind: don't allow cancelling bids, and simply prune all bids that
//! are not winning bids from the state.

use crate::{
    errors::AuctionError,
    processor::{AuctionData, Bid, BidderMetadata},
    utils::{assert_owned_by, create_or_allocate_account_raw},
    PREFIX,
};

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        program::invoke_signed,
        pubkey::Pubkey,
        system_instruction,
        sysvar::{clock::Clock, Sysvar},
    },
    std::mem,
};

/// Arguments for the PlaceBid instruction discriminant .
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlaceBidArgs {
    /// Size of the bid being placed. The user must have enough SOL to satisfy this amount.
    pub amount: u64,
    /// Resource being bid on.
    pub resource: Pubkey,
}

pub fn place_bid(program_id: &Pubkey, accounts: &[AccountInfo], args: PlaceBidArgs) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let bidder_act = next_account_info(account_iter)?;
    let auction_act = next_account_info(account_iter)?;
    let bidder_pot_act = next_account_info(account_iter)?;
    let bidder_meta_act = next_account_info(account_iter)?;
    let clock_sysvar = next_account_info(account_iter)?;
    let rent_act = next_account_info(account_iter)?;
    let system_account = next_account_info(account_iter)?;

    // Use the clock sysvar for timing the auction.
    let clock = Clock::from_account_info(clock_sysvar)?;

    // This path references an account to store the users bid SOL in, if the user wins the auction
    // this is claimed by the auction authority, otherwise the user can request to have the SOL
    // sent back.
    let pot_path = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_act.key.as_ref(),
        bidder_act.key.as_ref(),
    ];

    // Derive pot key, confirm it matches the users sent pot address.
    let (pot_key, pot_bump) = Pubkey::find_program_address(&pot_path, program_id);
    if pot_key != *bidder_pot_act.key {
        return Err(AuctionError::InvalidBidAccount.into());
    }

    // This path references an account to store the users bid SOL in, if the user wins the auction
    // this is claimed by the auction authority, otherwise the user can request to have the SOL
    // sent back.
    let meta_path = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_act.key.as_ref(),
        bidder_act.key.as_ref(),
        "metadata".as_bytes(),
    ];

    // Derive pot key, confirm it matches the users sent pot address.
    let (meta_key, meta_bump) = Pubkey::find_program_address(&meta_path, program_id);
    if meta_key != *bidder_meta_act.key {
        return Err(AuctionError::InvalidBidAccount.into());
    }

    // TODO: deal with rent and balance correctly, do this properly.
    if bidder_act.lamports() - args.amount <= 0 {
        return Err(AuctionError::BalanceTooLow.into());
    }

    // Pot path including the bump for seeds.
    let pot_seeds = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_act.key.as_ref(),
        bidder_act.key.as_ref(),
        &[pot_bump],
    ];

    // Allocate bid account, a token account to hold the resources.
    if true /* check account doesn't exist already */ {
        create_or_allocate_account_raw(
            *program_id,
            bidder_pot_act,
            rent_act,
            system_account,
            bidder_act,
            0,
            &pot_seeds,
        )?;
    }

    // Transfer SOL from the bidder's SOL account into their pot.
    invoke_signed(
        &system_instruction::transfer(bidder_act.key, &pot_key, args.amount),
        &[bidder_act.clone(), bidder_pot_act.clone()],
        &[&pot_seeds],
    )?;

    // Pot path including the bump for seeds.
    let meta_seeds = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_act.key.as_ref(),
        bidder_act.key.as_ref(),
        &[meta_bump],
    ];

    // Allocate a metadata account, to track the users state over time.
    if true /* check account doesn't exist already */ {
        create_or_allocate_account_raw(
            *program_id,
            bidder_pot_act,
            rent_act,
            system_account,
            bidder_act,
            mem::size_of::<BidderMetadata>(),
            &pot_seeds,
        )?;
    }

    // Load the auction and verify this bid is valid.
    let mut auction: AuctionData = try_from_slice_unchecked(&auction_act.data.borrow())?;

    // Do not allow bids post gap-time.
    if let Some(gap) = auction.gap_time {
        if clock.unix_timestamp - gap > 10 * 60 {
            return Err(AuctionError::BalanceTooLow.into());
        }
    }

    // Do not allow bids post end-time
    if let Some(end) = auction.end_time {
        if clock.unix_timestamp > end {
            return Err(AuctionError::BalanceTooLow.into());
        }
    }

    auction.last_bid = Some(clock.unix_timestamp);
    auction.bid_state.place_bid(Bid(pot_key, args.amount))?;
    auction.serialize(&mut *auction_act.data.borrow_mut())?;

    Ok(())
}

