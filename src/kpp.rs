use regex::Regex;
use common::Validate;
use error::Error;

pub struct Kpp {
    value: String,
}

impl Validate for Kpp {
    fn is_valid(&self) -> super::ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        let re = Regex::new(r"^[0-9]{4}[0-9A-Z]{2}[0-9]{3}$")?;

        match self.value.len() {
            9 => Ok(re.is_match(&self.value)),
            _ => Err(Error::InvalidCharacters {
                valid: "01234566789AZ".to_owned(),
            }),
        }
    }
}

impl Kpp {
    pub fn new(input: &str) -> Kpp {
        Kpp {
            value: input.into(),
        }
    }
}
