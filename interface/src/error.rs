//! Program error types.
#![allow(non_local_definitions)]

use {num_derive::FromPrimitive, pinocchio::program_error::ProgramError, thiserror::Error};

/// Errors that can be returned by the SBPF Verify program.
#[derive(Debug, Error, FromPrimitive)]
pub enum SBPFVerifyError {}

impl From<SBPFVerifyError> for ProgramError {
    fn from(e: SBPFVerifyError) -> Self {
        Self::Custom(e as u32)
    }
}
