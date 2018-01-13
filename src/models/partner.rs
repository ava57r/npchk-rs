use chrono::prelude::*;
use std::borrow::Cow;

/// Structure describes the data type, which is used by the server
#[derive(Debug)]
pub struct Partner<'a> {
    pub inn: Cow<'a, str>,
    pub kpp: Cow<'a, str>,
    pub dt: DateTime<Utc>,
    pub state: i32,
}

impl<'a> Partner<'a> {
    pub fn new<S>(inn: S, kpp: S, dt: DateTime<Utc>) -> Partner<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Partner {
            inn: inn.into(),
            kpp: kpp.into(),
            dt: dt,
            state: 0,
        }
    }
}
