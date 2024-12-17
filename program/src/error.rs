//! Program error types.

use {
    num_traits::{self, FromPrimitive},
    solana_program::{
        decode_error::DecodeError,
        program_error::{PrintProgramError, ProgramError},
    },
    solana_sbpf_verify::error::VerifyError,
    thiserror::Error,
};

/// Errors that can be returned by the SBPF Verify program.
#[derive(Debug, Error)]
pub enum SBPFVerifyError {
    /// ELF verification error.
    #[error(transparent)]
    VerifyError(#[from] VerifyError),
}

impl FromPrimitive for SBPFVerifyError {
    fn from_i64(n: i64) -> Option<Self> {
        VerifyError::from_i64(n).map(SBPFVerifyError::VerifyError)
    }

    fn from_u64(n: u64) -> Option<Self> {
        VerifyError::from_u64(n).map(SBPFVerifyError::VerifyError)
    }
}

impl From<SBPFVerifyError> for ProgramError {
    fn from(e: SBPFVerifyError) -> Self {
        match e {
            SBPFVerifyError::VerifyError(e) => e.into(),
        }
    }
}

impl<T> DecodeError<T> for SBPFVerifyError {
    fn type_of() -> &'static str {
        "SBPFVerifyError"
    }
}

impl PrintProgramError for SBPFVerifyError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            Self::VerifyError(e) => PrintProgramError::print::<VerifyError>(e),
        }
    }
}
