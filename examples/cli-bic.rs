//! Example to check Bank identification code

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
    const HELP: &'static str = "Usage: government_id command [arguments]...
    Commands:
        check BIC  Parameter BIC - Bank identification code contains
        help  - show this message.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => {
                if args.len() != 3 {
                    panic!("Usage: government_id check BIC");
                }
                check_bic(&args[2])
            }
            "help" => {
                println!("{}", HELP);
            }
            command @ _ => panic!(format!("Wrong command: {}", command)),
        },
        None => println!("{}", HELP),
    }
}
