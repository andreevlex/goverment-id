extern crate regex;

pub mod error;
pub mod tax_id;
pub mod reason_code;
pub mod bic;

pub use error::*;
pub use tax_id::*;
pub use reason_code::*;
pub use bic::*;

pub type ValidResult = std::result::Result<bool, error::Error>;

/// This contract describes the requirements to check for correctness.
pub trait Validate {
    fn is_valid(&self) -> ValidResult;
}

/// Checks that the string only digits
pub fn only_digits(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    for ch in input.chars() {
        if !ch.is_digit(10) {
            return false;
        }
    }
    true
}
