use chrono::prelude::*;
use super::partner::Partner;

#[derive(Debug)]
pub struct NdsResponse<'a> {
    pub dtact_fl: DateTime<Utc>,
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<Partner<'a>>,
}
