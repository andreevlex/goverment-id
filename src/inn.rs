use common::{only_digits, Validate};
use error::Error;

pub struct Inn {
    value: String,
    ratio: [u32; 11],
}

impl Inn {
    pub fn new(input: &str) -> Inn {
        Inn {
            value: input.into(),
            ratio: [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8],
        }
    }

    fn check_len12(&self) -> bool {
        let calc_num1 = self.check_digit(&self.ratio[1..]);
        let calc_num2 = self.check_digit(&self.ratio[..]);

        calc_num1 == get_digit(&self.value, 10) && calc_num2 == get_digit(&self.value, 11)
    }

    fn check_len10(&self) -> bool {
        let calc_num = self.check_digit(&self.ratio[2..]);

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

impl Validate for Inn {
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
