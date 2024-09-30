#![cfg(feature = "test-sbf")]

use {
    mollusk_svm::{result::Check, Mollusk},
    solana_sdk::{account::AccountSharedData, bpf_loader_upgradeable, pubkey::Pubkey},
};

pub fn setup() -> Mollusk {
    Mollusk::new(
        &solana_sbpf_verify_program::id(),
        "solana_sbpf_verify_program",
    )
}

#[test]
fn test_perf() {
    let mollusk = setup();

    // TODO: Set up proper buffer account(s).
    let buffer_address = Pubkey::new_unique();
    let buffer_account = AccountSharedData::new(100_000_000, 0, &bpf_loader_upgradeable::id());

    let instruction = solana_sbpf_verify_program::instruction::verify(&buffer_address);

    mollusk.process_and_validate_instruction(
        &instruction,
        &[(buffer_address, buffer_account)],
        &[
            Check::success(),
            Check::compute_units(375), // We'll see...
        ],
    );
}
