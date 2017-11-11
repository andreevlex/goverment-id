use chrono::prelude::*;

#[derive(Debug)]
pub struct NdsResponse<'a> {
    pub dtact_fl: DateTime<Utc>,
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<super::Partner<'a>>,
}
