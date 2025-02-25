//! Program error types.
#![allow(non_local_definitions)]

use {
    num_derive::FromPrimitive,
    solana_decode_error::DecodeError,
    solana_msg::msg,
    solana_program_error::{PrintProgramError, ProgramError},
    thiserror::Error,
};

/// Errors that can be returned by the SBPF Verify program.
#[derive(Debug, Error, FromPrimitive)]
pub enum SBPFVerifyError {}

impl PrintProgramError for SBPFVerifyError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<SBPFVerifyError> for ProgramError {
    fn from(e: SBPFVerifyError) -> Self {
        Self::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SBPFVerifyError {
    fn type_of() -> &'static str {
        "SBPFVerifyError"
    }
}
