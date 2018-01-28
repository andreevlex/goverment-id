extern crate regex;

pub mod error;
pub mod inn;
pub mod kpp;
pub mod bik;

pub use error::*;
pub use inn::*;
pub use kpp::*;
pub use bik::*;

pub type ValidResult = std::result::Result<bool, error::Error>;

pub trait Validate {
    fn is_valid(&self) -> ValidResult;
}

/// Проверяет, что в строке только цифры
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