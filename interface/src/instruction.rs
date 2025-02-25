//! Program instructions.

use {
    bytemuck::{Pod, Zeroable},
    pinocchio::program_error::ProgramError,
};

/// Instructions supported by the SBPF Verify program.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
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

unsafe impl Pod for SBPFVerifyInstruction {}
unsafe impl Zeroable for SBPFVerifyInstruction {}

impl SBPFVerifyInstruction {
    /// Interprets a
    /// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html) reference as
    /// bytes.
    pub fn bytes_of(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    /// Interprets a buffer as a
    /// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html) reference.
    pub fn interpret(data: &[u8]) -> Result<&Self, ProgramError> {
        bytemuck::try_from_bytes(data).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pack_unpack(instruction: SBPFVerifyInstruction) {
        let packed = instruction.bytes_of();
        let unpacked = SBPFVerifyInstruction::interpret(packed).unwrap();
        assert_eq!(&instruction, unpacked);
    }

    #[test]
    fn test_pack_unpack_verify() {
        test_pack_unpack(SBPFVerifyInstruction::Verify { elf_offset: 42 });
    }
}
