use chrono::prelude::*;
use super::partner::Partner;

/// Structure describes the type of data that the server sends to the client
#[derive(Debug)]
pub struct NdsResponse<'a> {
    /// Date on which relevant data for the individual entrepreneur,
    /// used to check.
    pub dtact_fl: DateTime<Utc>,
    /// Date on which relevant data for legal, used to check.
    pub dtact_ul: DateTime<Utc>,
    pub partners: Vec<Partner<'a>>,
}
