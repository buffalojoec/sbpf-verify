//! Solana eBPF ELF verifier library. Verify a Solana program ELF on-chain.
//!
//! Designed for use within on-chain programs, therefore to be as optimal on
//! compute unit usage as possible.

use spl_program_error::*;

/// Possible ELF verification errors.
#[spl_program_error]
pub enum VerifyError {
    /// This is a placeholder error.
    #[error("This is a placeholder error.")]
    Placeholder,
}

/// Verify a Solana program ELF.
pub fn verify_elf(_elf_bytes: &[u8]) -> Result<(), VerifyError> {
    Ok(())
}
