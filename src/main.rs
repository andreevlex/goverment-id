extern crate goverment_ids;

fn checkinn(s: &str) {
    if goverment_ids::inn::is_valid_inn(s) {
        println!("{}", "yes");
    }
    else {
        println!("{}", "no");
    }
}

fn main() {
	const HELP: &'static str = "Использование: govermentID команда [аргументы]...
    Команды:
        checkinn INN - проверить ИНН. Параметр INN - содержит ИНН
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