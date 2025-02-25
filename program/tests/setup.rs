#![cfg(feature = "test-sbf")]
#![allow(unused)]

use mollusk_svm::Mollusk;

// https://github.com/deanmlittle/sbpf-asm-noop
pub const ASM_NOOP_ELF: &[u8] = include_bytes!("./fixtures/asm_noop.so");
pub const SPL_MEMO_ELF: &[u8] = include_bytes!("./fixtures/spl_memo.so");
pub const SPL_NOOP_ELF: &[u8] = include_bytes!("./fixtures/spl_noop.so");
pub const SPL_TOKEN_ELF: &[u8] = include_bytes!("./fixtures/spl_token.so");

pub fn setup() -> Mollusk {
    Mollusk::new(&sbpf_verify_interface::id(), "sbpf_verify")
}
