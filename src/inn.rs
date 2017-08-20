
pub fn is_valid_inn(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    if !only_digits(input) {
        return false;
    }

    match input.len() {
        12 => check_inn_len12(input),
        10 => check_inn_len10(input),
        _ => false
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

fn check_digit(input: &str, ratio: &[u32]) -> u32 {
    let mut sum = 0;
    
    for i in 0..ratio.len() {
        let num = get_digit(input, i);
        sum += num * ratio[i];
    }
    sum % 11 % 10
}

fn check_inn_len12(input: &str) -> bool {
    let ratio1:[u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let calc_num1 = check_digit(input, &ratio1);
    
    let ratio2:[u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];    
    let calc_num2 = check_digit(input, &ratio2);
    
    calc_num1 == get_digit(input, 10) && calc_num2 == get_digit(input, 11)
}

fn check_inn_len10(input: &str) -> bool {
    let ratio:[u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    
    let calc_num = check_digit(input, &ratio);
    
    calc_num == get_digit(input, 9)
}