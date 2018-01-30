use super::{ValidResult, Validate};
use error::Error;

/// Проверка `банковского идентификационного кода`
pub struct BankIdentificationCode {
    value: String,
}

impl BankIdentificationCode {
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
        match create_bank_identification_code("000000000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
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

}
