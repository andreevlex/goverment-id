pub use common::{only_digits, ValidResult, Validate};

pub struct Bik {
    value: String,
}

impl Bik {
    pub fn new(input: &str) -> Bik {
        Bik {
            value: input.to_string(),
        }
    }
}

impl Validate for Bik {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
            return Err("БИК пуст".to_string());
        }

        if !only_digits(&self.value) {
            return Err(
                "БИК должен состоять только из цифр".to_string(),
            );
        }

        match self.value.len() {
            9 => Ok(true),
            _ => Err(
                "БИК может состоять только из 9 знаков".to_string(),
            ),
        }
    }
}
