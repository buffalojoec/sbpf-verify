//! Program entrypoint.

use {
    crate::processor,
    pinocchio::{account_info::AccountInfo, program_entrypoint, pubkey::Pubkey, ProgramResult},
};

program_entrypoint!(process);

fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    processor::process(program_id, accounts, input)
}
