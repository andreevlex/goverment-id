use super::{ValidResult, Validate};
use error::Error;

/// Проверка `банковского идентификационного кода`
pub struct Bik {
    value: String,
}

impl Bik {
    pub fn new(input: &str) -> Self {
        Bik {
            value: input.into(),
        }
    }
}

impl Validate for Bik {
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

    fn create_bik(s: &str) -> Bik {
        Bik::new(s)
    }

    #[test]
    fn test_empty_bik() {
        match create_bik("").is_valid() {
            Err(error::Error::Empty) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_bik_8_numbers() {
        match create_bik("01234567").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_bik_9_zeros() {
        match create_bik("000000000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_bik_10_numbers() {
        match create_bik("0123456789").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_bik_with_litters() {
        match create_bik("0000AZ000").is_valid() {
            Err(error::Error::ExpectedNumbersOnly) => assert!(true),
            _ => assert!(false),
        };
    }

}
