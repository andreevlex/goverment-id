use super::{ValidResult, Validate};
use error::Error;
use regex::Regex;

/// Проверка `кода причины постановки на учёт`
pub struct ReasonRegistrationCode {
    value: String,
}

impl ReasonRegistrationCode {
    pub fn new(input: &str) -> Self {
        ReasonRegistrationCode {
            value: input.into(),
        }
    }
}

impl Validate for ReasonRegistrationCode {
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

    fn create_reason_registration_code(s: &str) -> ReasonRegistrationCode {
        ReasonRegistrationCode::new(s)
    }

    #[test]
    fn test_invalid_reason_registration_code_8_numbers() {
        match create_reason_registration_code("01234567").is_valid() {
            Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_reason_registration_code_9_zeros() {
        match create_reason_registration_code("000000000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_reason_registration_code_10_numbers() {
        match create_reason_registration_code("0123456789").is_valid() {
            Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_reason_registration_code_with_upper_case_litters() {
        match create_reason_registration_code("0000AZ000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_reason_registration_code_with_lower_case_litters() {
        match create_reason_registration_code("0000Az000").is_valid() {
            Ok(false) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_reason_registration_code_with_not_alphabet_chars() {
        match create_reason_registration_code("0000A-000").is_valid() {
            Ok(false) => assert!(true),
            _ => assert!(false),
        };
    }

}
