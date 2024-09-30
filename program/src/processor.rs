//! Program processor.

use {
    crate::instruction::SBPFVerifyInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    },
};

fn process_verify(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_info = next_account_info(accounts_iter)?;
    let elf_bytes = buffer_info.try_borrow_data()?;

    solana_sbpf_verify::verify_elf(&elf_bytes)?;

    Ok(())
}

/// Processes a
/// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = SBPFVerifyInstruction::unpack(input)?;
    match instruction {
        SBPFVerifyInstruction::Verify => {
            msg!("Instruction: Verify");
            process_verify(program_id, accounts)
        }
    }
}
