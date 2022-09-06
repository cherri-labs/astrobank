/*******************************************************

 Defines all given entrypoints to the program and
 calls the appropriate process_instruction.

 ******************************************************/

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use crate::processor as proc;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    if data.len() == 0 {
        msg!("Error: no entrypoint specified. Empty instruction data!");
        return Err(ProgramError::InvalidInstructionData);
    }

    /* 0: withdraw      *
     * 1: drain account */
    if data[0] == 0 {
        return proc::withdraw(
            program_id,
            accounts,
            &data[1..data.len()],
        );
    } else if data[0] == 1 {
        return proc::drain_account(
            program_id,
            accounts,
            &data[1..data.len()],
        );
    }

    msg!("Error in data array (element #0): unassigned u8 `data[0]` entrypoint id. Couldn't find specified entrypoint.");
    Err(ProgramError::InvalidInstructionData)
}
