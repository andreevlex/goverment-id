use common::Validate;

use inn::Inn;

#[test]
fn test_empty_inn() {
    assert!(Inn::new("").is_valid() == Err("ИНН пуст".to_string()));
}

#[test]
fn test_invalid_inn_9zeros() {
    assert!(
        Inn::new("000000000").is_valid()
            == Err("ИНН должен быть длиной 10 или 12 цифр".to_string())
    );
}

#[test]
fn test_valid_inn_10zeros() {
    assert!(Inn::new("0000000000").is_valid() == Ok(true));
}

#[test]
fn test_invalid_inn_11zeros() {
    assert!(
        Inn::new("00000000000").is_valid()
            == Err("ИНН должен быть длиной 10 или 12 цифр".to_string())
    );
}

#[test]
fn test_valid_inn_12zeros() {
    assert!(Inn::new("000000000000").is_valid() == Ok(true));
}

#[test]
fn test_invalid_inn_too_short() {
    assert!(
        Inn::new("772053").is_valid()
            == Err("ИНН должен быть длиной 10 или 12 цифр".to_string())
    );
}

#[test]
fn test_valid_inn_10_numbers() {
    assert!(Inn::new("7827004526").is_valid() == Ok(true));
}

#[test]
fn test_invalid_check_digit_inn_10_numbers() {
    assert!(Inn::new("7827004527").is_valid() == Ok(false));
}

#[test]
fn test_valid_inn_12_numbers() {
    assert!(Inn::new("760307073214").is_valid() == Ok(true));
}

#[test]
fn test_invalid_check_digit_inn_12_numbers() {
    assert!(Inn::new("760307073217").is_valid() == Ok(false));
}

#[test]
fn test_invalid_inn_with_litters() {
    assert!(
        Inn::new("782f004526").is_valid()
            == Err("ИНН должен состоять только из цифр".to_string())
    );
}

use kpp::Kpp;

#[test]
fn test_invalid_kpp_7_numbers() {
    assert!(
        Kpp::new("01234567").is_valid()
            == Err(
                "КПП может состоять только из 9 знаков (цифр или заглавных букв латинского алфавита от A до Z)"
                    .to_string()
            )
    );
}

#[test]
fn test_valid_kpp_9_zeros() {
    assert!(Kpp::new("000000000").is_valid() == Ok(true));
}

#[test]
fn test_invalid_kpp_10_numbers() {
    assert!(
        Kpp::new("0123456789").is_valid()
            == Err(
                "КПП может состоять только из 9 знаков (цифр или заглавных букв латинского алфавита от A до Z)"
                    .to_string()
            )
    );
}

#[test]
fn test_valid_kpp_with_upper_case_litters() {
    assert!(Kpp::new("0000AZ000").is_valid() == Ok(true));
}

#[test]
fn test_invalid_kpp_with_lower_case_litters() {
    assert!(Kpp::new("0000Az000").is_valid() == Ok(false));
}

#[test]
fn test_invalid_kpp_with_not_alfabet_chars() {
    assert!(Kpp::new("0000A-000").is_valid() == Ok(false));
}

use bik::Bik;

#[test]
fn test_empty_bik() {
    assert!(Bik::new("").is_valid() == Err("БИК пуст".to_string()));
}

#[test]
fn test_invalid_bik_7_numbers() {
    assert!(
        Bik::new("01234567").is_valid()
            == Err(
                "БИК может состоять только из 9 знаков".to_string()
            )
    );
}

#[test]
fn test_valid_bik_9_zeros() {
    assert!(Bik::new("000000000").is_valid() == Ok(true));
}

#[test]
fn test_invalid_bik_10_numbers() {
    assert!(
        Bik::new("0123456789").is_valid()
            == Err(
                "БИК может состоять только из 9 знаков".to_string()
            )
    );
}

#[test]
fn test_invalid_bik_with_litters() {
    assert!(
        Bik::new("0000AZ000").is_valid()
            == Err("БИК должен состоять только из цифр".to_string())
    );
}
