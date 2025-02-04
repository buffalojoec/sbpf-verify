//! Program entrypoint.

use {
    crate::{error::SBPFVerifyError, processor},
    solana_account_info::AccountInfo,
    solana_program_error::{PrintProgramError, ProgramResult},
    solana_pubkey::Pubkey,
};

solana_program_entrypoint::entrypoint!(process);

fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    processor::process(program_id, accounts, input)
        .inspect_err(PrintProgramError::print::<SBPFVerifyError>)
}
