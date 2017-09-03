extern crate goverment_ids;

use goverment_ids::{inn, kpp};
use goverment_ids::common::Validate;

fn print_result(res: bool) {
    if res {
        println!("{}", "yes");
    }
    else {
        println!("{}", "no");
    }
}

fn checkinn(s: &str) {
    let value = inn::Inn::new(s);
    match value.is_valid() {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn checkkpp(s: &str) {
    let value = kpp::Kpp::new(s);
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
                                    checkinn(&args[3])
                                },
                                "kpp" => {
                                    if args.len() != 4 {
                                        panic!("Использование: govermentID check kpp KPP");
                                    }
                                    checkkpp(&args[3])
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