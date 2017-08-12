
pub fn is_valid_inn(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    if !only_digits(input) {
        return false;
    }

    let len = input.len();
    
    match len {
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

fn check_inn_len12(input: &str) -> bool {
    let ratio1:[u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let mut sum1 = 0;
    
    for i in 0..ratio1.len() {
        let num = get_digit(input, i);
        sum1 += num * ratio1[i];
    }
    let calc_num1 = sum1 % 11;
    
    let ratio2:[u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];    
    let mut sum2 = 0;    
    for i in 0..ratio2.len() {
        let num = get_digit(input, i);
        sum2 += num * ratio2[i];
    }
    let calc_num2 = sum2 % 11;
    
    let ctrl_num1 = get_digit(input, 10);
    let ctrl_num2 = get_digit(input, 11);

    calc_num1 == ctrl_num1 && calc_num2 == ctrl_num2
}

fn check_inn_len10(input: &str) -> bool {
    let ratio:[u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    let mut sum = 0;
    
    for i in 0..ratio.len() {
        let num = get_digit(input, i);
        sum += num * ratio[i];
    }
    let calc_num = sum % 11;
    let ctrl_num = get_digit(input, 9);

    calc_num == ctrl_num
}