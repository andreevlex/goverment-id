extern crate goverment_ids;

fn print_result(res: bool) {
    if res {
        println!("{}", "yes");
    }
    else {
        println!("{}", "no");
    }
}

fn checkinn(s: &str) {
    match goverment_ids::inn::is_valid_inn(s) {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn checkkpp(s: &str) {
    match goverment_ids::kpp::is_valid_kpp(s) {
        Ok(res) => print_result(res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn main() {
	const HELP: &'static str = "Использование: govermentID команда [аргументы]...
    Команды:
        checkinn INN - проверить ИНН. Параметр INN - содержит Идентификационный номер налогоплательщика (ИНН)
        checkkpp KPP - проверить КПП. Параметр KPP - содержит Код причины постановки на учет (КПП)
        help  - показать это сообщение.";
    
	let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "checkinn" => {
                    if args.len() != 3 {
                            panic!("Использование: govermentID checkinn INN");
                    }
                    checkinn(&args[2])
                },
                "checkkpp" => {
                    if args.len() != 3 {
                            panic!("Использование: govermentID checkkpp KPP");
                    }
                    checkkpp(&args[2])
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