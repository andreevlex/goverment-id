pub trait Validate {
    fn is_valid(&self) -> super::ValidResult;
}

pub fn only_digits(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    for ch in input.chars() {
        if !ch.is_digit(10) {
            return false;
        }
    }
    true
}
