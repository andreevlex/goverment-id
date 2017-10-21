use regex::Regex;
pub use common::{ValidResult, Validate};

pub struct Kpp {
    value: String,
}

impl Validate for Kpp {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
            return Err("КПП пуст".to_string());
        }

        let re = Regex::new(r"^[0-9]{4}[0-9A-Z]{2}[0-9]{3}$").unwrap();

        match self.value.len() {
            9 => Ok(re.is_match(&self.value)),
            _ => Err(
                "КПП может состоять только из 9 знаков (цифр или заглавных букв латинского алфавита от A до Z)"
                    .to_string(),
            ),
        }
    }
}

impl Kpp {
    pub fn new(input: &str) -> Kpp {
        Kpp {
            value: input.to_string(),
        }
    }
}
