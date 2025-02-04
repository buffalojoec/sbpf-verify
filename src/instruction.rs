//! Program instructions.

use {
    solana_instruction::{AccountMeta, Instruction},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

/// Instructions supported by the SBPF Verify program.
#[derive(Debug, PartialEq)]
pub enum SBPFVerifyInstruction {
    /// Verify a program ELF.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[ ]` Buffer account.
    /// 1. `[ ]` ...
    Verify {
        /// Offset into the buffer account where the ELF is stored.
        ///
        /// This input allows other types of buffer accounts, besides the
        /// canonical loaders, to be compatible with this program.
        elf_offset: u64,
    },
}

impl SBPFVerifyInstruction {
    /// Packs a [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html) into a
    /// buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            SBPFVerifyInstruction::Verify { elf_offset } => {
                let mut buf = Vec::with_capacity(9);
                buf.push(0);
                buf.extend_from_slice(&elf_offset.to_le_bytes());
                buf
            }
        }
    }

    /// Unpacks a buffer into a
    /// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            Some((&0, rest)) if rest.len() == 8 => {
                let elf_offset = u64::from_le_bytes(rest.try_into().unwrap());
                Ok(SBPFVerifyInstruction::Verify { elf_offset })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Creates a [Verify](enum.SBPFVerifyInstruction.html) instruction.
pub fn verify(buffer_address: &Pubkey, elf_offset: u64) -> Instruction {
    let accounts = vec![AccountMeta::new(*buffer_address, false)];
    let data = SBPFVerifyInstruction::Verify { elf_offset }.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pack_unpack(instruction: SBPFVerifyInstruction) {
        let packed = instruction.pack();
        let unpacked = SBPFVerifyInstruction::unpack(&packed).unwrap();
        assert_eq!(instruction, unpacked);
    }

    #[test]
    fn test_pack_unpack_verify() {
        test_pack_unpack(SBPFVerifyInstruction::Verify { elf_offset: 42 });
    }
}
