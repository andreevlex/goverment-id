use common::Validate;

use inn::Inn;
use error;

#[test]
fn test_empty_inn() {
    match Inn::new("").is_valid() {
        Err(error::Error::Empty) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_9_zeros() {
    match Inn::new("000000000").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_10zeros() {
    match Inn::new("0000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_11zeros() {
    match Inn::new("00000000000").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_12zeros() {
    match Inn::new("000000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_too_short() {
    match Inn::new("772053").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_10_numbers() {
    match Inn::new("7827004526").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_check_digit_inn_10_numbers() {
    match Inn::new("7827004527").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_12_numbers() {
    match Inn::new("760307073214").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_check_digit_inn_12_numbers() {
    match Inn::new("760307073217").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_with_litters() {
    match Inn::new("782f004526").is_valid() {
        Err(error::Error::ExpectedNumbersOnly) => assert!(true),
        _ => assert!(false),
    };
}

use kpp::Kpp;

#[test]
fn test_invalid_kpp_8_numbers() {
    match Kpp::new("01234567").is_valid() {
        Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_kpp_9_zeros() {
    match Kpp::new("000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_10_numbers() {
    match Kpp::new("0123456789").is_valid() {
        Err(error::Error::InvalidCharacters { valid: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_kpp_with_upper_case_litters() {
    match Kpp::new("0000AZ000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_with_lower_case_litters() {
    match Kpp::new("0000Az000").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_kpp_with_not_alfabet_chars() {
    match Kpp::new("0000A-000").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

use bik::Bik;

#[test]
fn test_empty_bik() {
    match Bik::new("").is_valid() {
        Err(error::Error::Empty) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_bik_8_numbers() {
    match Bik::new("01234567").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_bik_9_zeros() {
    match Bik::new("000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_bik_10_numbers() {
    match Bik::new("0123456789").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_bik_with_litters() {
    match Bik::new("0000AZ000").is_valid() {
        Err(error::Error::ExpectedNumbersOnly) => assert!(true),
        _ => assert!(false),
    };
}
