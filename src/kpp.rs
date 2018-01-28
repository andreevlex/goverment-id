use super::{ValidResult, Validate};
use error::Error;
use regex::Regex;

/// Проверка `кода причины постановки на учёт`
pub struct Kpp {
    value: String,
}

impl Kpp {
    pub fn new(input: &str) -> Self {
        Kpp {
            value: input.into(),
        }
    }
}

impl Validate for Kpp {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        let re = Regex::new(r"^[0-9]{4}[0-9A-Z]{2}[0-9]{3}$")?;

        match self.value.len() {
            9 => Ok(re.is_match(&self.value)),
            _ => Err(Error::InvalidCharacters {
                valid: "01234566789AZ".to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use error;
    use super::*;

    fn create_kpp(s: &str) -> Kpp {
    Kpp::new(s)
}

#[test]
fn test_invalid_kpp_8_numbers() {
    match create_kpp("01234567").is_valid() {
        Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_kpp_9_zeros() {
    match create_kpp("000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_10_numbers() {
    match create_kpp("0123456789").is_valid() {
        Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_kpp_with_upper_case_litters() {
    match create_kpp("0000AZ000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_with_lower_case_litters() {
    match create_kpp("0000Az000").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_with_not_alphabet_chars() {
    match create_kpp("0000A-000").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

}