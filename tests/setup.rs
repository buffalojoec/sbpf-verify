#![cfg(feature = "test-sbf")]
#![allow(unused)]

use mollusk_svm::Mollusk;

pub fn setup() -> Mollusk {
    Mollusk::new(&sbpf_verify::id(), "sbpf_verify")
}
