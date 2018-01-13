extern crate regex;

pub mod checkers;
pub mod error;
#[cfg(test)]
mod unit_test;

pub type ValidResult = std::result::Result<bool, error::Error>;
