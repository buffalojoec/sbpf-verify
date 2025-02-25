//! Program entrypoint.

use {
    crate::processor,
    pinocchio::{entrypoint::InstructionContext, lazy_program_entrypoint, ProgramResult},
};

lazy_program_entrypoint!(process);

fn process(context: InstructionContext) -> ProgramResult {
    processor::process(context)
}
