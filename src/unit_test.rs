use checkers::*;
use error;

fn create_inn(s: &str) -> Checker<Inn> {
    Checker::new(s).into()
}

fn create_kpp(s: &str) -> Checker<Kpp> {
    Checker::new(s).into()
}

fn create_bik(s: &str) -> Checker<Bik> {
    Checker::new(s).into()
}

#[test]
fn test_empty_inn() {
    match create_inn("").is_valid() {
        Err(error::Error::Empty) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_9_zeros() {
    match create_inn("000000000").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_10zeros() {
    match create_inn("0000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_11zeros() {
    match create_inn("00000000000").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_12zeros() {
    match create_inn("000000000000").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_too_short() {
    match create_inn("772053").is_valid() {
        Err(error::Error::WrongLength { length: _ }) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_10_numbers() {
    match create_inn("7827004526").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_check_digit_inn_10_numbers() {
    match create_inn("7827004527").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_valid_inn_12_numbers() {
    match create_inn("760307073214").is_valid() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_check_digit_inn_12_numbers() {
    match create_inn("760307073217").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
}

#[test]
fn test_invalid_inn_with_litters() {
    match create_inn("782f004526").is_valid() {
        Err(error::Error::ExpectedNumbersOnly) => assert!(true),
        _ => assert!(false),
    };
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
fn test_invalid_kpp_with_not_alfabet_chars() {
    match create_kpp("0000A-000").is_valid() {
        Ok(false) => assert!(true),
        _ => assert!(false),
    };
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
