use chrono::{DateTime, Utc};

pub struct Partner {
    pub inn: String,
    pub kpp: String,
    pub dt: DateTime<Utc>,
    pub state: i32,
}

impl Partner {
    pub fn new( inn: String, kpp: String, dt: DateTime<Utc>) -> Partner {
        Partner {
            inn: inn,
            kpp: kpp,
            dt: dt,
            state: 0,
        }
    }
}