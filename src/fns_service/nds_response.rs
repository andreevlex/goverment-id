use chrono::prelude::*;
use fns_service::Partner;

#[derive(Debug)]
pub struct NdsResponse {
    pub dtact_fl: DateTime<Utc>,
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<Partner>,
}
