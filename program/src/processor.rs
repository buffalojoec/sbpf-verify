//! Program processor.

use {
    pinocchio::{
        account_info::AccountInfo, entrypoint::InstructionContext, msg,
        program_error::ProgramError, ProgramResult,
    },
    sbpf_verify_interface::instruction::SBPFVerifyInstruction,
    solana_sbpf::{
        elf::Executable,
        elf_parser::{
            types::{Elf64Ehdr, Elf64Phdr},
            Elf64, ElfParserError,
        },
        program::{FunctionRegistry, SBPFVersion},
        verifier::{RequisiteVerifier, Verifier},
    },
    std::{mem, sync::Arc},
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

fn quick_parse_text_bytes(elf_bytes: &[u8]) -> Result<&[u8], ElfParserError> {
    // Taken from: <>
    let (_, file_header) = Elf64::parse_file_header(elf_bytes)?;
    let program_header_table_range = mem::size_of::<Elf64Ehdr>()
        ..mem::size_of::<Elf64Phdr>()
            .saturating_mul(file_header.e_phnum as usize)
            .saturating_add(mem::size_of::<Elf64Ehdr>());

    // Taken from: <>
    let program_header_table =
        Elf64::slice_from_bytes::<Elf64Phdr>(elf_bytes, program_header_table_range.clone())?;

    // Taken from: <>
    let bytecode_header = &program_header_table[0];

    // Taken from: <>
    let text_section_range = bytecode_header.file_range().unwrap_or_default();

    Ok(&elf_bytes[text_section_range])
}

/// Processes a [Verify](enum.SBPFVerifyInstruction.html) instruction.
fn process_verify(buffer_info: AccountInfo, elf_offset: u64) -> ProgramResult {
    let data = buffer_info.try_borrow_data()?;
    let elf = &data[elf_offset as usize..];

    msg!("Obtained ELF from account info");

    let text_bytes = quick_parse_text_bytes(elf).map_err(|_| ProgramError::InvalidArgument)?;
    let loader = program_runtime_environment::empty_loader();
    let sbpf_version = SBPFVersion::V0;

    let executable =
        Executable::<program_runtime_environment::MockedInvokeContext>::new_from_text_bytes(
            text_bytes,
            Arc::clone(&loader),
            sbpf_version,
            FunctionRegistry::<usize>::default(),
        )
        .map_err(|_| ProgramError::InvalidArgument)?;

    msg!("Parsed ELF components");

    RequisiteVerifier::verify(
        text_bytes,
        loader.get_config(),
        sbpf_version,
        executable.get_function_registry(),
        loader.get_function_registry(),
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    msg!("Verified ELF");

    Ok(())
}

/// Processes a
/// [SBPFVerifyInstruction](enum.SBPFVerifyInstruction.html).
pub fn process(mut context: InstructionContext) -> ProgramResult {
    // SAFETY: The buffer account should definitely be provided, and should not
    // be duplicated at index 0.
    let buffer_account = context.next_account()?.assume_account();

    // SAFETY: All accounts required by the instruction have been read above.
    let input = context.instruction_data()?;

    match SBPFVerifyInstruction::interpret(input)? {
        SBPFVerifyInstruction::Verify { elf_offset } => {
            msg!("Instruction: Verify");
            process_verify(buffer_account, *elf_offset)
        }
    }
}
