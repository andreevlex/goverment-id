
pub use common::{Validate, ValidResult};

pub struct Inn {
    value: String,
}

impl Inn {
    pub fn new(input: &str) -> Inn {
        Inn { value: input.to_string() }
    } 

    fn check_inn_len12(&self, input: &str) -> bool {
        let ratio1:[u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
        let calc_num1 = self.check_digit(input, &ratio1);
    
        let ratio2:[u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];    
        let calc_num2 = self.check_digit(input, &ratio2);
    
        calc_num1 == get_digit(input, 10) && calc_num2 == get_digit(input, 11)
    }

    fn check_inn_len10(&self, input: &str) -> bool {
        let ratio:[u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    
        let calc_num = self.check_digit(input, &ratio);
    
        calc_num == get_digit(input, 9)
    }

    fn check_digit(&self, input: &str, ratio: &[u32]) -> u32 {
        let mut sum = 0;
    
        for i in 0..ratio.len() {
            let num = get_digit(input, i);
            sum += num * ratio[i];
        }
        sum % 11 % 10
    }
}

fn only_digits(input: &str) -> bool {    
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

fn get_digit(input: &str, n: usize) -> u32 {
    let ch:char = match input.chars().nth(n) {
        Some(x) => x,
        None => '0',
    };

    match ch.to_digit(10) {
        Some(d) => d,
        None => 0,
    }
}

impl Validate for Inn {
    fn is_valid(&self) -> ValidResult {
        if self.value.is_empty() {
        return Err("ИНН пуст".to_string());
        }

        if !only_digits(&self.value) {
            return Err("ИНН должен состоять только из цифр".to_string());
        }

        match self.value.len() {
            12 => Ok(self.check_inn_len12(&self.value)),
            10 => Ok(self.check_inn_len10(&self.value)),
            _ => Err("ИНН должен быть длиной 10 или 12 цифр".to_string())
        }
    }
}