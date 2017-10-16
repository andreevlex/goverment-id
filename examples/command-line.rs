extern crate goverment_id;

use goverment_id::{inn, kpp, bik};
use goverment_id::common::Validate;

fn print_result(res: bool) {
    if res {
        println!("{}", "yes");
    }
    else {
        println!("{}", "no");
    }
}

fn check_inn(s: &str) {
    let value = inn::Inn::new(s);
    match value.is_valid() {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn check_kpp(s: &str) {
    let value = kpp::Kpp::new(s);
    match value.is_valid() {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn check_bik(s: &str) {
    let value = bik::Bik::new(s);
    match value.is_valid() {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}


fn main() {
	const HELP: &'static str = "Использование: govermentID команда [аргументы]...
    Команды:
        check inn INN - проверить ИНН. Параметр INN - содержит Идентификационный номер налогоплательщика (ИНН)
        check kpp KPP - проверить КПП. Параметр KPP - содержит Код причины постановки на учет (КПП)
        check bik BIK - проверить КПП. Параметр BIK - содержит Банковский идентификационный код (БИК)
        help  - показать это сообщение.";
    
	let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "check" => {
                    match args.get(2) {
                        Some(flag) => {
                            match flag.as_ref() {
                                "inn" => {
                                    if args.len() != 4 {
                                        panic!("Использование: govermentID check inn INN");
                                    }
                                    check_inn(&args[3])
                                },
                                "kpp" => {
                                    if args.len() != 4 {
                                        panic!("Использование: govermentID check kpp KPP");
                                    }
                                    check_kpp(&args[3])
                                },
                                "bik" => {
                                    if args.len() != 4 {
                                        panic!("Использование: govermentID check bik BIK");
                                    }
                                    check_bik(&args[3])
                                },
                                param @ _  => panic!(
                                    format!("Неправильный параметр команды check: {}", param))    
                            }
                        },
                        None => panic!("Отсуствует обязательный параметр")
                    }
                },
               "help" => {
                    println!("{}", HELP);
                },
                command @ _  => panic!(
                    format!("Неправильная команда: {}", command))
            }
        }
        None => println!("{}", HELP),
    }
}