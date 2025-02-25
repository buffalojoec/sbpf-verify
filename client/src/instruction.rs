//! Program instructions.

use {
    sbpf_verify_interface::instruction::SBPFVerifyInstruction,
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

/// Creates a `Verify` instruction.
pub fn verify(buffer_address: &Pubkey, elf_offset: u64) -> Instruction {
    let accounts = vec![AccountMeta::new(*buffer_address, false)];
    let data = SBPFVerifyInstruction::Verify { elf_offset }.pack();
    Instruction::new_with_bytes(sbpf_verify_interface::id(), &data, accounts)
}
