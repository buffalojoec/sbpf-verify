//! Program processor.

use {
    crate::instruction::SBPFVerifyInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        bpf_loader_upgradeable::{UpgradeableLoaderState, ID as LOADER_V3_ID},
        entrypoint::ProgramResult,
        loader_v4::ID as LOADER_V4_ID,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

const BUFFER_METADATA_LEN: usize = UpgradeableLoaderState::size_of_buffer_metadata();

fn verify_buffer_account(buffer_info: &AccountInfo) -> Result<(), ProgramError> {
    // Buffer account should be owned by one of the two non-deprecated loaders.
    if !buffer_info.owner.eq(&LOADER_V3_ID) && !buffer_info.owner.eq(&LOADER_V4_ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Buffer account should have proper metadata.
    if let Ok(UpgradeableLoaderState::Buffer { .. }) =
        bincode::deserialize(&buffer_info.data.borrow()[..BUFFER_METADATA_LEN])
    {
        return Ok(());
    }

    Err(ProgramError::InvalidAccountData)
}

fn process_verify(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_info = next_account_info(accounts_iter)?;
    verify_buffer_account(buffer_info)?;

    let elf_bytes = &buffer_info.data.borrow()[BUFFER_METADATA_LEN..];
    solana_sbpf_verify::verify_elf(elf_bytes)?;

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
