//! Program instructions.

use {
    sbpf_verify_interface::instruction::SBPFVerifyInstruction,
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

/// Creates a `Verify` instruction.
pub fn verify(buffer_address: &Pubkey, elf_offset: u64) -> Instruction {
    let accounts = vec![AccountMeta::new(*buffer_address, false)];
    let input = SBPFVerifyInstruction::Verify { elf_offset };
    Instruction::new_with_bytes(sbpf_verify_interface::id(), input.bytes_of(), accounts)
}
