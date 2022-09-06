/*******************************************************

 Defines all entrypoint functions to process any given
 input instruction data.

 ******************************************************/

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
    sysvar::Sysvar,
    clock::{Clock, UnixTimestamp},
};
use crate::accounts;
use crate::accounts::rent_exemption;

// lock period length in seconds
const LOCK_TIME: UnixTimestamp = 256 * 24 * 60 * 60;

////////////////////////
/* custom error codes */
////////////////////////

/* 0: active lock period                 *
 * 1: incorrect associated token account *
 * 2: incorrect spl token amount         *
 * 3: incorrect spl token mint           */

/////////////////////
/* program structs */
/////////////////////

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Account {
    pub creation: UnixTimestamp,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Request {
    pub seed: String,
    pub amount: u64,
}

/////////////////
/* entrypoints */
/////////////////

pub fn withdraw<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // writeable
    let writing_account = accounts::writeable(
        next_account_info(accounts_iter)?,
        &program_id
    )?;

    // signer
    let recipient_account = accounts::signer(
        next_account_info(accounts_iter)?
    )?;

    // create new withdrawal request
    let mut request = Request::try_from_slice(&data)
        .expect("Error: failed to deserialize instruction data.");

    // find program address
    let program_address = Pubkey::create_with_seed(
        &recipient_account.key,
        &request.seed,
        program_id
    )?;

    // verify account ownership
    if program_address != *writing_account.key {
        // not account owner
        msg!("Error: only the account owner can request a withdrawal.");
        return Err(ProgramError::IllegalOwner);
    }

    if **writing_account.lamports.borrow() < request.amount {
        // insufficient balance for withdrawal
        msg!("Error: insufficient balance for withdrawal.\n
            Current account balance: {}.", **writing_account.lamports.borrow());
        return Err(ProgramError::InsufficientFunds);
    } else if **writing_account.lamports.borrow() - rent_exemption(writing_account) < request.amount {
        // insufficient balance for rent exemption
        request.amount = **writing_account.lamports.borrow();
    }

    // transfer lamports to recipient
    **writing_account.try_borrow_mut_lamports()? -= request.amount;
    **recipient_account.try_borrow_mut_lamports()? += request.amount;

    Ok(())
}

pub fn drain_account<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // writeable
    let writing_account = accounts::writeable(
        next_account_info(accounts_iter)?,
        &program_id
    )?;

    // owner // token account
    let owner_account = accounts::owner_signer(
        next_account_info(accounts_iter)?,
        next_account_info(accounts_iter)?
    )?;

    // deserialize account data
    let account_data = Account::try_from_slice(&writing_account.try_borrow_data()?)
        .expect("Error: failed to deserialize account data.");

    // create new transfer request
    let mut request = Request::try_from_slice(&data)
        .expect("Error: failed to deserialize instruction data.");

    // get clock time
    let clock = Clock::get()?;
    let time_left = LOCK_TIME - (clock.unix_timestamp - account_data.creation);

    if time_left > 0 && clock.unix_timestamp > account_data.creation {
        // lock period running (not up or not started yet)
        msg!("Error: `drain_account` cannot force withdrawal before lock period is up.\n
            Current Unix timestamp: {}\n,
            Account creation time: {}\n,
            Time left in lock period: {}", clock.unix_timestamp, account_data.creation, time_left);
        return Err(ProgramError::Custom(0));
    } else if **writing_account.lamports.borrow() < request.amount {
        // insufficient balance for withdrawal
        request.amount = **writing_account.lamports.borrow();
        msg!("Warning: client balance smaller than request amount {}.\n
              Defaulting to current account balance: {}.",
             request.amount,
             **writing_account.lamports.borrow());
    } else if **writing_account.lamports.borrow() - rent_exemption(writing_account) < request.amount {
        // insufficient balance for rent exemption
        request.amount = **writing_account.lamports.borrow();
        msg!("Warning: insufficient funds remaining for rent exemption.\n
            Balance needed for rent exemption: {}.\n
            Defaulting to current account balance: {}.", rent_exemption(writing_account), **writing_account.lamports.borrow());
    }

    // transfer lamports
    **writing_account.try_borrow_mut_lamports()? -= request.amount;
    **owner_account.try_borrow_mut_lamports()? += request.amount;

    Ok(())
}
