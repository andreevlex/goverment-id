extern crate chrono;
extern crate goverment_id;

use goverment_id::fns_service::*;
use chrono::prelude::*;

fn main() {
    let mut partners: Vec<Partner> = vec![];
    partners.push(Partner::new(
        "4205036750".to_string(),
        "420501001".to_string(),
        Utc::now(),
    ));
    partners.push(Partner::new(
        "6648185610".to_string(),
        "662301001".to_string(),
        Utc::now(),
    ));

    match check_fns(&partners) {
        Ok(rsp) => println!("{:?}", rsp),
        Err(e) => println!("Error {:?}", e),
    }
}
