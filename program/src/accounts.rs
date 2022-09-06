/*******************************************************

 Module for reading and handling input accounts.

 ******************************************************/

use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    msg,
    program_pack::Pack,
};
use spl_token::state::Account;
use spl_associated_token_account::get_associated_token_address;
use crate::owner_token;

//////////////////////
/* helper functions */
//////////////////////

pub fn rent_exemption(account: &AccountInfo) -> u64 {
    Rent::get().unwrap().minimum_balance(account.data_len())
}

//////////////////////
/* account handling */
//////////////////////

pub fn writeable<'a>(
    account: &'a AccountInfo<'a>,
    program_id: &Pubkey
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    if account.owner != program_id {
        // error: not program account
        msg!("Error: account {} owned by {}, not by program.", account.key, account.owner);
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(account)
}

pub fn signer<'a>(
    account: &'a AccountInfo<'a>
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    if !account.is_signer {
        // error: not signer
        msg!("Error: failed to retrieve account signature. Account isn't signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok(account)
}

pub fn owner<'a>(
    owner_account: &'a AccountInfo<'a>,
    spl_account: &'a AccountInfo<'a>
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let spl_token_account_data = spl_account.try_borrow_data()?;
    let spl_token_account = Account::unpack(&spl_token_account_data)?;

    if spl_token_account.mint == owner_token::get_mint_address() { // mint
        if spl_token_account.amount >= owner_token::MIN_AMOUNT { // balance
            if spl_token_account.owner == *owner_account.key { // account
                return Ok(owner_account);
            } else {
                // error: incorrect spl token account: not associated token account
                msg!("Error: account is not associated token account.\n
                    {} is not expected owner: {}.", spl_token_account.owner, *owner_account.key);
                return Err(ProgramError::Custom(1));
            }
        } else {
            // error: incorrect token amount
            msg!("Error: insufficient token balance.\n
                {} isn't enough. Expected: {}.", spl_token_account.amount, owner_token::MIN_AMOUNT);
            return Err(ProgramError::Custom(2));
        }
    } else {
        // error: incorrect token mint
        msg!("Error: incorrect token account mint address.\n
            {} isn't expected: {}.", spl_token_account.mint, owner_token::get_mint_address());
        return Err(ProgramError::Custom(3));
    }
}

pub fn owner_signer<'a>(
    owner_account: &'a AccountInfo<'a>,
    spl_account: &'a AccountInfo<'a>
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    signer(
        owner(
            &owner_account,
            &spl_account
        )?
    )
}

pub fn _rent_exempt<'a>(
    account: &'a AccountInfo<'a>
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    if **account.lamports.borrow() < rent_exemption(account) {
        // error: insufficient balance for rent exemption
        msg!("Error: account balance insufficient for rent exemption.\n
            Balance needed for rent exemption: {}.", rent_exemption(&account));
        return Err(ProgramError::AccountNotRentExempt);
    }

    Ok(account)
}

pub fn _rent_exempt_writeable<'a>(
    account: &'a AccountInfo<'a>,
    program_id: &Pubkey
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    _rent_exempt(
        writeable(
            &account,
            &program_id
        )?
    )
}
