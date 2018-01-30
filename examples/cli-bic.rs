extern crate government_id;

use government_id::*;

fn check_bic(s: &str) {
    let value = BankIdentificationCode::new(s);
    match value.is_valid() {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn main() {
    const HELP: &'static str = "Использование: government_id команда [аргументы]...
    Команды:
        check bic BIC - проверить КПП. Параметр BIC - содержит Банковский идентификационный код (БИК)
        help  - показать это сообщение.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => match args.get(2) {
                Some(flag) => match flag.as_ref() {
                    "bic" => {
                        if args.len() != 4 {
                            panic!("Использование: government_id check bic BIC");
                        }
                        check_bic(&args[3])
                    }
                    param @ _ => panic!(format!(
                        "Неправильный параметр команды check: {}",
                        param
                    )),
                },
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
