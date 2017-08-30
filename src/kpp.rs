extern crate regex;

pub type ValidResult = Result<bool, String>;

pub fn is_valid_kpp(input: &str) -> ValidResult {
    if input.is_empty() {
        return Err("КПП пуст".to_string());
    }
    
    match input.len() {
        9 => Ok(check_kpp(input)),
        _ => Err("КПП может состоять только из 9 знаков (цифр или заглавных букв латинского алфавита от A до Z)".to_string())
    }

}

fn check_kpp(input: &str) -> bool
{
    let re = regex::Regex::new(r"^[0-9]{4}[0-9A-Z]{2}[0-9]{3}$").unwrap();
    
    re.is_match(input)
}