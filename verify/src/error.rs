//! Possible ELF verification errors.

use {
    crate::ebpf,
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Possible ELF verification errors.
#[derive(Debug, Error, FromPrimitive)]
pub enum VerifyError {
    /// ProgramLengthNotMultiple
    #[error("Program length must be a multiple of {} octets", ebpf::INSN_SIZE)]
    ProgramLengthNotMultiple,
    /// NoProgram
    #[error("No program provided")]
    NoProgram,
}

impl From<VerifyError> for ProgramError {
    fn from(value: VerifyError) -> Self {
        ProgramError::Custom(value as u32)
    }
}

impl<T> DecodeError<T> for VerifyError {
    fn type_of() -> &'static str {
        "VerifyError"
    }
}

impl PrintProgramError for VerifyError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            VerifyError::ProgramLengthNotMultiple => {
                solana_program::msg!(
                    "Program length must be a multiple of {} octets",
                    ebpf::INSN_SIZE
                )
            }
            VerifyError::NoProgram => solana_program::msg!("No program provided"),
        }
    }
}
