use super::{ValidResult, Validate};
use error::Error;

/// Проверка `Идентификационного номера налогоплательщика`
pub struct TaxpayerIdentificationNumber {
    value: String,
}

impl TaxpayerIdentificationNumber {
    const RATIO: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];

    pub fn new(input: &str) -> Self {
        TaxpayerIdentificationNumber {
            value: input.into(),
        }
    }

    /// Проверяет `ИНН` индивидуального предпринимателя
    fn check_len12(&self) -> bool {
        let calc_num1 = self.check_digit(&TaxpayerIdentificationNumber::RATIO[1..]);
        let calc_num2 = self.check_digit(&TaxpayerIdentificationNumber::RATIO[..]);

        calc_num1 == get_digit(&self.value, 10) && calc_num2 == get_digit(&self.value, 11)
    }

    /// Проверяет `ИНН` юридического лица
    fn check_len10(&self) -> bool {
        let calc_num = self.check_digit(&TaxpayerIdentificationNumber::RATIO[2..]);

        calc_num == get_digit(&self.value, 9)
    }

    /// Рассчитывает контрольную цифру
    fn check_digit(&self, r: &[u32]) -> u32 {
        let mut sum = 0;

        for i in 0..r.len() {
            let num = get_digit(&self.value, i);
            sum += num * r[i];
        }
        sum % 11 % 10
    }
}

/// Получает цифру из строки по индексу
fn get_digit(input: &str, n: usize) -> u32 {
    match input.chars().nth(n) {
        Some(ch) => match ch.to_digit(10) {
            Some(d) => d,
            None => 0,
        },
        None => 0,
    }
}

impl Validate for TaxpayerIdentificationNumber {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        if !super::only_digits(&self.value) {
            return Err(Error::ExpectedNumbersOnly);
        }

        match self.value.len() {
            12 => Ok(self.check_len12()),
            10 => Ok(self.check_len10()),
            1...9 => Err(Error::WrongLength { length: 10 }),
            11 => Err(Error::WrongLength { length: 12 }),
            _ => Err(Error::WrongLength { length: 12 }),
        }
    }
}

#[cfg(test)]
mod tests {
    use error;
    use super::*;

    fn create_taxpayer_identification_number(s: &str) -> TaxpayerIdentificationNumber {
        TaxpayerIdentificationNumber::new(s)
    }

    #[test]
    fn test_empty_taxpayer_identification_number() {
        match create_taxpayer_identification_number("").is_valid() {
            Err(error::Error::Empty) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_taxpayer_identification_number_9_zeros() {
        match create_taxpayer_identification_number("000000000").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_taxpayer_identification_number_10zeros() {
        match create_taxpayer_identification_number("0000000000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_taxpayer_identification_number_11zeros() {
        match create_taxpayer_identification_number("00000000000").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_taxpayer_identification_number_12zeros() {
        match create_taxpayer_identification_number("000000000000").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_taxpayer_identification_number_too_short() {
        match create_taxpayer_identification_number("772053").is_valid() {
            Err(error::Error::WrongLength { length: _ }) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_taxpayer_identification_number_10_numbers() {
        match create_taxpayer_identification_number("7827004526").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_check_digit_taxpayer_identification_number_10_numbers() {
        match create_taxpayer_identification_number("7827004527").is_valid() {
            Ok(false) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_valid_taxpayer_identification_number_12_numbers() {
        match create_taxpayer_identification_number("760307073214").is_valid() {
            Ok(true) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_check_digit_taxpayer_identification_number_12_numbers() {
        match create_taxpayer_identification_number("760307073217").is_valid() {
            Ok(false) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_invalid_taxpayer_identification_number_with_litters() {
        match create_taxpayer_identification_number("782f004526").is_valid() {
            Err(error::Error::ExpectedNumbersOnly) => assert!(true),
            _ => assert!(false),
        };
    }

}
