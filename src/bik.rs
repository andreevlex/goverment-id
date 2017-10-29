use common::{only_digits, Validate};
use error::Error;

pub struct Bik {
    value: String,
}

impl Bik {
    pub fn new(input: &str) -> Bik {
        Bik {
            value: input.into(),
        }
    }
}

impl Validate for Bik {
    fn is_valid(&self) -> super::ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        if !only_digits(&self.value) {
            return Err(Error::ExpectedNumbersOnly);
        }

        match self.value.len() {
            9 => Ok(true),
            _ => Err(Error::WrongLength { length: 9 }),
        }
    }
}
