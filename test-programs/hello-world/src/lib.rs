//! Hello, world!

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

solana_program::entrypoint!(process);

pub fn process(_program_id: &Pubkey, _accounts: &[AccountInfo], _input: &[u8]) -> ProgramResult {
    msg!("Hello, world!");
    Ok(())
}
