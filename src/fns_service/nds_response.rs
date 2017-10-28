use chrono::prelude::*;

#[derive(Debug)]
pub struct NdsResponse {
    pub dtact_fl: DateTime<Utc>,
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<super::Partner>,
}
