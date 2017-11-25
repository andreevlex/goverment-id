use regex;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Empty,
    ExpectedNumbersOnly,
    WrongLength { length: i32 },
    InvalidCharacters { valid: String },
    RegexError(regex::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Empty => write!(f, "Пустое значение не допустимо"),
            Error::ExpectedNumbersOnly => write!(
                f,
                "Значение должны состоять только из цифр"
            ),
            Error::WrongLength { ref length } => write!(
                f,
                "Значение должно быть длинной {}",
                length
            ),
            Error::InvalidCharacters { ref valid } => write!(
                f,
                "Значение должно состоять из следующих символов '{}'",
                valid
            ),
            Error::RegexError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Empty => "Пустое значение не допустимо",
            Error::ExpectedNumbersOnly => {
                "Значение должны состоять только из цифр"
            }
            Error::WrongLength { length: _ } => {
                "Длина значения не соотвествует требованиям"
            }
            Error::InvalidCharacters { valid: _ } => {
                "В значении присуствуют недопустимые символы"
            }
            Error::RegexError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Empty => None,
            Error::ExpectedNumbersOnly => None,
            Error::WrongLength { length: _ } => None,
            Error::InvalidCharacters { valid: _ } => None,
            Error::RegexError(ref e) => e.cause(),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(other: regex::Error) -> Error {
        Error::RegexError(other)
    }
}
