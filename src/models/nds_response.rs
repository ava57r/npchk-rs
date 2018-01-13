use chrono::prelude::*;
use super::partner::Partner;

/// Structure describes the type of data that the server sends to the client
#[derive(Debug)]
pub struct NdsResponse<'a> {
    pub dtact_fl: DateTime<Utc>,
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<Partner<'a>>,
}
