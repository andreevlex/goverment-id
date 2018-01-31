//! Example usage check the reason code of registration
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
    const HELP: &'static str = "Usage: government_id command [arguments]...
    Commands:
        check RRC Parameter RRC - contains the reason Code of registration
        help  - show this message.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => {
                if args.len() != 3 {
                    panic!("Usage: government_id check RRC");
                }
                check_reason_registration_code(&args[2])
            },
            "help" => {
                println!("{}", HELP);
            }
            command @ _ => panic!(format!(
                "Wrong command: {}",
                command
            )),
        },
        None => println!("{}", HELP),
    }
}
