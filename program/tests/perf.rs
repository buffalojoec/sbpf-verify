#![cfg(feature = "test-sbf")]

use {
    mollusk_svm::{result::Check, Mollusk},
    solana_sdk::{
        account::{Account, AccountSharedData},
        bpf_loader_upgradeable::{self, UpgradeableLoaderState},
        pubkey::Pubkey,
        rent::Rent,
    },
};

const BUFFER_METADATA_LEN: usize = UpgradeableLoaderState::size_of_buffer_metadata();

fn setup() -> Mollusk {
    Mollusk::new(
        &solana_sbpf_verify_program::id(),
        "solana_sbpf_verify_program",
    )
}

fn buffer_account(elf: &[u8]) -> AccountSharedData {
    let space = BUFFER_METADATA_LEN.saturating_add(elf.len());
    let lamports = Rent::default().minimum_balance(space);
    let mut data = vec![0; space];
    bincode::serialize_into(
        &mut data[..],
        &UpgradeableLoaderState::Buffer {
            authority_address: None,
        },
    )
    .unwrap();
    data[BUFFER_METADATA_LEN..].copy_from_slice(elf);
    AccountSharedData::from(Account {
        lamports,
        data,
        owner: bpf_loader_upgradeable::id(),
        ..Account::default()
    })
}

fn test_elf_hello_world() -> Vec<u8> {
    mollusk_svm::file::load_program_elf("test_program_hello_world")
}

#[test]
fn test_perf() {
    let mollusk = setup();

    let elf = test_elf_hello_world();

    let buffer_address = Pubkey::new_unique();
    let buffer_account = buffer_account(&elf);

    let instruction = solana_sbpf_verify_program::instruction::verify(&buffer_address);

    mollusk.process_and_validate_instruction(
        &instruction,
        &[(buffer_address, buffer_account)],
        &[
            Check::success(),
            // Check::compute_units(375), // We'll see...
        ],
    );
}
