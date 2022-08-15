/*******************************************************

 Module for reading program related pubkeys.

 ******************************************************/

use std::str::FromStr;
use solana_program::{
    pubkey::Pubkey,
};

// expected owner token amount
pub const MIN_AMOUNT: u64 = 1;

// expected owner token mint
pub fn get_mint_address() -> Pubkey {
    Pubkey::from_str("CVEzUhRyUwbxwjDWP6RhZqjGPiVnLtxc2oNDBJck8chn").unwrap()
}
