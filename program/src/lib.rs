//! SBPF Verify program.
#![deny(missing_docs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod processor;
