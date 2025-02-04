//! Solana eBPF ELF verifier library. Verify a Solana program ELF on-chain.
//!
//! Designed for use within on-chain programs, therefore to be as optimal on
//! compute unit usage as possible.

pub mod ebpf;
pub mod elf;
pub mod error;
pub mod verify;

use crate::error::VerifyError;

/// Verify a Solana program ELF.
pub fn verify_elf(_elf_bytes: &[u8]) -> Result<(), VerifyError> {
    // 1. Configure program runtime environment (syscalls).
    // 2. Load bytes into `Elf` object.
    // 3. Verify ELF.
    Ok(())
}
