#![cfg(feature = "test-sbf")]

mod setup;

use {
    mollusk_svm::result::Check, sbpf_verify_client::instruction::verify, solana_account::Account,
    solana_instruction::error::InstructionError, solana_pubkey::Pubkey,
};

fn buffer_account(elf: &[u8]) -> Account {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Mock buffer metadata
    data.extend_from_slice(elf);
    Account {
        lamports: 0,
        data,
        owner: Pubkey::new_unique(),
        ..Account::default()
    }
}

#[test]
fn asm_noop() {
    let mollusk = setup::setup();

    let buffer_address = Pubkey::new_unique();
    let buffer_account = buffer_account(setup::ASM_NOOP_ELF);

    mollusk.process_and_validate_instruction(
        &verify(&buffer_address, /* elf_offset */ 8),
        &[(buffer_address, buffer_account)],
        &[
            // Success.
            // ASM Noop ELF size: 336
            Check::success(),
            Check::compute_units(5_922), // <-- CUs used
        ],
    );
}

#[test]
fn spl_noop() {
    let mollusk = setup::setup();

    let buffer_address = Pubkey::new_unique();
    let buffer_account = buffer_account(setup::SPL_NOOP_ELF);

    mollusk.process_and_validate_instruction(
        &verify(&buffer_address, /* elf_offset */ 8),
        &[(buffer_address, buffer_account)],
        &[
            // Busts the program's heap.
            // SPL Noop ELF size: 41_056
            Check::instruction_err(InstructionError::ProgramFailedToComplete),
        ],
    );
}

#[test]
fn spl_memo() {
    let mollusk = setup::setup();

    let buffer_address = Pubkey::new_unique();
    let buffer_account = buffer_account(setup::SPL_MEMO_ELF);

    mollusk.process_and_validate_instruction(
        &verify(&buffer_address, /* elf_offset */ 8),
        &[(buffer_address, buffer_account)],
        &[
            // Busts the program's heap.
            // SPL Memo ELF size: 74_800
            Check::instruction_err(InstructionError::ProgramFailedToComplete),
        ],
    );
}

#[test]
fn spl_token() {
    let mollusk = setup::setup();

    let buffer_address = Pubkey::new_unique();
    let buffer_account = buffer_account(setup::SPL_TOKEN_ELF);

    mollusk.process_and_validate_instruction(
        &verify(&buffer_address, /* elf_offset */ 8),
        &[(buffer_address, buffer_account)],
        &[
            // Busts the program's heap.
            // SPL Token ELF size: 134_080
            Check::instruction_err(InstructionError::ProgramFailedToComplete),
        ],
    );
}
