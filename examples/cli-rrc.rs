//! Пример использования проверки кода причины постановки на учет
extern crate government_id;

use government_id::*;

fn check_reason_registration_code(s: &str) {
    let value = ReasonRegistrationCode::new(s);
    match value.is_valid() {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn main() {
    const HELP: &'static str = "Использование: government_id команда [аргументы]...
    Команды:
        check reason_code RRC - проверить КПП. Параметр KPP - содержит Код причины постановки на учет (КПП)
        help  - показать это сообщение.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => match args.get(2) {
                Some(flag) => {
                    match flag.as_ref() {
                        "reason_code" => {
                            if args.len() != 4 {
                                panic!("Использование: government_id check reason_code RRC");
                            }
                            check_reason_registration_code(&args[3])
                        }
                        param @ _ => panic!(format!(
                            "Неправильный параметр команды check: {}",
                            param
                        )),
                    }
                }
                None => panic!("Отсутствует обязательный параметр"),
            },
            "help" => {
                println!("{}", HELP);
            }
            command @ _ => panic!(format!(
                "Неправильная команда: {}",
                command
            )),
        },
        None => println!("{}", HELP),
    }
}
