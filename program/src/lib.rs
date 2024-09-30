//! Solana SBPF Verify Program.
//!
//! On-chain SBPF program for verifying program ELFs.
#![allow(unexpected_cfgs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("SBPFVerify111111111111111111111111111111111");
