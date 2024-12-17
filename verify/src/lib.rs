//! Solana eBPF ELF verifier library. Verify a Solana program ELF on-chain.
//!
//! Designed for use within on-chain programs, therefore to be as optimal on
//! compute unit usage as possible.

pub mod error;

use crate::error::VerifyError;

/// Verify a Solana program ELF.
pub fn verify_elf(_elf_bytes: &[u8]) -> Result<(), VerifyError> {
    // TODO! Optimize!

    // TODO: These should probably be configurable.
    // let _feature_set = FeatureSet::all_enabled();
    // let _compute_budget = ComputeBudget::default();

    // Register syscalls.
    // let _environment = create_program_runtime_environment_v1(
    //     &feature_set,
    //     &compute_budget,
    //     /* reject_deployment_of_broken_elfs */ true,
    //     /* debugging_features */ false,
    // )
    // .unwrap();

    // Boo, nothing else in these steps can be imported in a BPF context.
    // We'll need a BPF-compatible ELF verifier...

    Ok(())
}
