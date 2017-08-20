use inn;

#[test]
fn test_empty_inn() {
    assert!(inn::is_valid_inn("") == Err("ИНН пуст".to_string()));
}

#[test]
fn test_invalid_inn_9zeros() {
    assert!(inn::is_valid_inn("000000000") == Err("ИНН должен быть длиной 10 или 12 цифр".to_string()));
}

#[test]
fn test_valid_inn_10zeros() {
    assert!(inn::is_valid_inn("0000000000") == Ok(true));
}

#[test]
fn test_invalid_inn_11zeros() {
    assert!(inn::is_valid_inn("00000000000") == Err("ИНН должен быть длиной 10 или 12 цифр".to_string()));
}

#[test]
fn test_valid_inn_12zeros() {
    assert!(inn::is_valid_inn("000000000000") == Ok(true));
}

#[test]
fn test_invalid_inn_too_short() {
    assert!(inn::is_valid_inn("772053") == Err("ИНН должен быть длиной 10 или 12 цифр".to_string()));
}

#[test]
fn test_valid_inn_10_numbers() {
    assert!(inn::is_valid_inn("7827004526") == Ok(true));
}

#[test]
fn test_invalid_check_digit_inn_10_numbers() {
    assert!(inn::is_valid_inn("7827004527") == Ok(false));
}

#[test]
fn test_valid_inn_12_numbers() {
    assert!(inn::is_valid_inn("760307073214") == Ok(true));
}

#[test]
fn test_invalid_check_digit_inn_12_numbers() {
    assert!(inn::is_valid_inn("760307073217") == Ok(false));
}

#[test]
fn test_invalid_inn_with_litters() {
    assert!(inn::is_valid_inn("782f004526") == Err("ИНН должен состоять только из цифр".to_string()));
}