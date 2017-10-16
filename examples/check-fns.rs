extern crate goverment_id;
extern crate chrono;

use goverment_id::fns_service::*;
use chrono::prelude::*;

fn main() {
    let mut partners: Vec<Partner> = vec![];
    partners.push(Partner::new(
            "4205036750".to_string(),
            "420501001".to_string(),
            Utc::now())
            );

    match check_fns(&partners) {
        Ok(rsp) => println!("{:?}", rsp),
        Err(_) => panic!("Что-то пошло не по плану"),
    }
}