//! Program processor.

use {
    crate::instruction::SBPFVerifyInstruction,
    solana_account_info::AccountInfo,
    solana_msg::msg,
    solana_program_error::{ProgramError, ProgramResult},
    solana_pubkey::Pubkey,
    solana_sbpf::{elf::Executable, verifier::RequisiteVerifier},
};

mod program_runtime_environment {
    //! This whole module is mocked-out right now, since we need a way to keep
    //! the list of syscalls this program recognizes in sync with that of the
    //! Solana version running on the cluster.

    use {
        solana_sbpf::{
            program::BuiltinProgram,
            vm::{Config, ContextObject},
        },
        std::sync::Arc,
    };

    pub struct MockedInvokeContext {
        meter: u64,
    }

    impl ContextObject for MockedInvokeContext {
        fn trace(&mut self, _state: [u64; 12]) {
            // Stubbed
        }

        fn consume(&mut self, amount: u64) {
            self.meter -= amount;
        }

        fn get_remaining(&self) -> u64 {
            self.meter
        }
    }

    pub fn empty_loader() -> Arc<BuiltinProgram<MockedInvokeContext>> {
        Arc::new(BuiltinProgram::new_loader(Config::default()))
    }
}

/// Processes a [Verify](enum.SBPFVerifyInstruction.html) instruction.
fn process_verify(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    elf_offset: u64,
) -> ProgramResult {
    let buffer_info = accounts.first().ok_or(ProgramError::NotEnoughAccountKeys)?;

    let data = buffer_info.try_borrow_data()?;
    let elf = &data[elf_offset as usize..];

    msg!("Obtained ELF from account info");

    let executable = Executable::<program_runtime_environment::MockedInvokeContext>::load(
        elf,
        program_runtime_environment::empty_loader(),
    )
    .map_err(|_| ProgramError::InvalidArgument)?;

    msg!("Loaded ELF into an executable");

    executable
        .verify::<RequisiteVerifier>()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    msg!("Verified ELF");

    Ok(())
}

/// Processes a
/// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match SBPFVerifyInstruction::unpack(input)? {
        SBPFVerifyInstruction::Verify { elf_offset } => {
            msg!("Instruction: Verify");
            process_verify(program_id, accounts, elf_offset)
        }
    }
}
