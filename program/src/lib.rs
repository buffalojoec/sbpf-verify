//! SBPF Verify program.
#![deny(missing_docs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_package_metadata::declare_id_with_package_metadata!("solana.program-id");
