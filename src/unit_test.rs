use inn;

#[test]
fn test_invalid_inn_zeros() {
    assert!(inn::is_valid_inn("000000000") == false);
}

#[test]
fn test_invalid_inn_too_short() {
    assert!(inn::is_valid_inn("772053") == false);
}

#[test]
fn test_valid_inn() {
    assert!(inn::is_valid_inn("7827004526") == true);
}

#[test]
fn test_invalid_inn_with_litters() {
    assert!(inn::is_valid_inn("782f004526") == false);
}