use super::{ValidResult, Validate};
use error::Error;

/// This structure describes a Bank identification code
/// and enables you to return information about its properties.
/// To check whether it is correct.
///
/// # Examples
///
/// ```rust
///
/// use government_id::*;
/// let bic: BankIdentificationCode = "000000000".to_owned().into();
/// assert!(bic.is_valid().unwrap());
///
/// ```
///
pub struct BankIdentificationCode {
    value: String,
}

impl BankIdentificationCode {
    /// Creates a new `BankIdentificationCode`
    pub fn new(input: &str) -> Self {
        BankIdentificationCode {
            value: input.into(),
        }
    }
}

impl Validate for BankIdentificationCode {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        if !super::only_digits(&self.value) {
            return Err(Error::ExpectedNumbersOnly);
        }

        match self.value.len() {
            9 => Ok(true),
            _ => Err(Error::WrongLength { length: 9 }),
        }
    }
}

impl From<String> for BankIdentificationCode {
    fn from(other: String) -> BankIdentificationCode {
        BankIdentificationCode { value: other }
    }
}

#[cfg(test)]
mod tests {
    use error;
    use super::*;

    fn create_bank_identification_code(s: &str) -> BankIdentificationCode {
        BankIdentificationCode::new(s)
    }

    #[test]
    fn test_empty_bank_identification_code() {
        match create_bank_identification_code("").is_valid() {
            Err(error::Error::Empty) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_bank_identification_code_8_numbers() {
        match create_bank_identification_code("01234567").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_bank_identification_code_9_zeros() {
        assert!(
            create_bank_identification_code("000000000")
                .is_valid()
                .unwrap()
        );
    }

    #[test]
    fn test_invalid_bank_identification_code_10_numbers() {
        match create_bank_identification_code("0123456789").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_bank_identification_code_with_litters() {
        match create_bank_identification_code("0000AZ000").is_valid() {
            Err(error::Error::ExpectedNumbersOnly) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_convert_from_string() {
        let bic: BankIdentificationCode = "000000000".to_owned().into();
        assert!(match bic.is_valid() {
            Ok(true) => true,
            _ => false,
        })
    }

}
