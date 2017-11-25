use error::Error;
use regex::Regex;

pub trait Validate {
    fn is_valid(&self) -> super::ValidResult;
}

pub struct Checker<S> {
    value: String,
    concrete_checker: S,
}

pub struct Inn {
    ratio: [u32; 11],
}

pub struct AbstractChecker;

impl Checker<AbstractChecker> {
    pub fn new(input: &str) -> Self {
        Checker {
            value: input.into(),
            concrete_checker: AbstractChecker,
        }
    }
}

impl From<Checker<AbstractChecker>> for Checker<Inn> {
    fn from(other: Checker<AbstractChecker>) -> Checker<Inn> {
        let checker = Inn {
            ratio: [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8],
        };

        Checker {
            value: other.value.into(),
            concrete_checker: checker,
        }
    }
}

impl Checker<Inn> {
    fn check_len12(&self) -> bool {
        let calc_num1 = self.check_digit(&self.concrete_checker.ratio[1..]);
        let calc_num2 = self.check_digit(&self.concrete_checker.ratio[..]);

        calc_num1 == get_digit(&self.value, 10) && calc_num2 == get_digit(&self.value, 11)
    }

    fn check_len10(&self) -> bool {
        let calc_num = self.check_digit(&self.concrete_checker.ratio[2..]);

        calc_num == get_digit(&self.value, 9)
    }

    fn check_digit(&self, ratio: &[u32]) -> u32 {
        let mut sum = 0;

        for i in 0..ratio.len() {
            let num = get_digit(&self.value, i);
            sum += num * ratio[i];
        }
        sum % 11 % 10
    }
}

fn get_digit(input: &str, n: usize) -> u32 {
    let ch: char = match input.chars().nth(n) {
        Some(x) => x,
        None => '0',
    };

    match ch.to_digit(10) {
        Some(d) => d,
        None => 0,
    }
}

impl Validate for Checker<Inn> {
    fn is_valid(&self) -> super::ValidResult {
        if self.value.is_empty() {
            return Err(Error::Empty);
        }

        if !only_digits(&self.value) {
            return Err(Error::ExpectedNumbersOnly);
        }

        match self.value.len() {
            12 => Ok(self.check_len12()),
            10 => Ok(self.check_len10()),
            1...9 => Err(Error::WrongLength { length: 10 }),
            11 => Err(Error::WrongLength { length: 12 }),
            _ => Err(Error::WrongLength { length: 12 }),
        }
    }
}

pub struct Kpp;

impl From<Checker<AbstractChecker>> for Checker<Kpp> {
    fn from(other: Checker<AbstractChecker>) -> Checker<Kpp> {
        Checker {
            value: other.value.into(),
            concrete_checker: Kpp,
        }
    }
}

impl Validate for Checker<Kpp> {
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

pub struct Bik;

impl From<Checker<AbstractChecker>> for Checker<Bik> {
    fn from(other: Checker<AbstractChecker>) -> Checker<Bik> {
        Checker {
            value: other.value.into(),
            concrete_checker: Bik,
        }
    }
}

impl Validate for Checker<Bik> {
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
