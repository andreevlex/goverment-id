use super::{ValidResult, Validate};
use error::Error;
use regex::Regex;

/// This structure describes a reason code of registration
/// and allows to obtain information about its properties.
/// To check whether it is correct.
/// ///
/// # Examples
///
/// ```rust
///
/// use government_id::*;
/// let rrc: ReasonRegistrationCode = "0000AZ000".to_owned().into();
/// assert!(rrc.is_valid().unwrap());
///
/// ```
///
pub struct ReasonRegistrationCode {
    value: String,
}

impl ReasonRegistrationCode {
    /// Creates a new `ReasonRegistrationCode`
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

impl From<String> for ReasonRegistrationCode {
    fn from(other: String) -> ReasonRegistrationCode {
        ReasonRegistrationCode { value: other }
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
        assert!(
            create_reason_registration_code("000000000")
                .is_valid()
                .unwrap()
        );
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
        assert!(
            create_reason_registration_code("0000AZ000")
                .is_valid()
                .unwrap()
        );
    }

    #[test]
    fn test_invalid_reason_registration_code_with_lower_case_litters() {
        assert!(
            create_reason_registration_code("0000Az000")
                .is_valid()
                .unwrap() == false
        );
    }

    #[test]
    fn test_invalid_reason_registration_code_with_not_alphabet_chars() {
        assert!(
            create_reason_registration_code("0000A-000")
                .is_valid()
                .unwrap() == false
        );
    }

    #[test]
    fn test_convert_from_string() {
        let rrc: ReasonRegistrationCode = "0000AZ000".to_owned().into();
        assert!(match rrc.is_valid() {
            Ok(true) => true,
            _ => false,
        })
    }

}
