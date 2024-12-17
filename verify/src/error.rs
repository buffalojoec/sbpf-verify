//! Possible ELF verification errors.

use {
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
    /// This is a placeholder error.
    #[error("This is a placeholder error.")]
    Placeholder,
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
            VerifyError::Placeholder => {
                println!("This is a placeholder error.");
            }
        }
    }
}
