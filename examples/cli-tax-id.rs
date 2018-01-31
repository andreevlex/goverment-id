//! Example to verify taxpayer identification numbers

extern crate government_id;

use government_id::*;

fn check_tax_id(s: &str) {
    let value = TaxpayerIdentificationNumber::new(s);
    match value.is_valid() {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn main() {
    const HELP: &'static str = "Usage: government_id command [arguments]...
    Commands:
        check TAX_ID Option TAX_ID - contains taxpayer Identification number
        help  - show this message.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => {
                if args.len() != 3 {
                    panic!("Usage: government_id check TAX_ID");
                }
                check_tax_id(&args[2])
            }
            "help" => {
                println!("{}", HELP);
            }
            command @ _ => panic!(format!("Wrong command: {}", command)),
        },
        None => println!("{}", HELP),
    }
}
