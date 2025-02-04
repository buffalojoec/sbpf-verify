//! Solana program ELF verification.

use crate::{ebpf, error::VerifyError};

fn check_prog_len(prog: &[u8]) -> Result<(), VerifyError> {
    if prog.len() % ebpf::INSN_SIZE != 0 {
        return Err(VerifyError::ProgramLengthNotMultiple);
    }
    if prog.is_empty() {
        return Err(VerifyError::NoProgram);
    }
    Ok(())
}

pub fn verify(prog: &[u8]) -> Result<(), VerifyError> {
    check_prog_len(prog)?;

    Ok(())
}
