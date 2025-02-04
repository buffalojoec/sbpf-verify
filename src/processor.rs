//! Program processor.

use {
    crate::instruction::SBPFVerifyInstruction, solana_account_info::AccountInfo, solana_msg::msg,
    solana_program_error::ProgramResult, solana_pubkey::Pubkey,
};

/// Processes a [Verify](enum.SBPFVerifyInstruction.html) instruction.
fn process_verify(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _elf_offset: u64,
) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match SBPFVerifyInstruction::unpack(input)? {
        SBPFVerifyInstruction::Verify { elf_offset } => {
            msg!("Instruction: Verify");
            process_verify(program_id, accounts, elf_offset)
        }
    }
}
