use chrono::prelude::*;
use std::borrow::Cow;

/// Structure describes the data type, which is used by the server
#[derive(Debug)]
pub struct Partner<'a> {
    /// Taxpayer identification number
    pub inn: Cow<'a, str>,
    /// The reason code of registration
    pub kpp: Cow<'a, str>,
    /// Date on which the requested information
    pub dt: DateTime<Utc>,
    /// Validation status
    /// The following options
    /// 
    /// For a legal entity:
    /// 
    /// 0 - the Taxpayer was in the unified state register of taxpayers
    /// and had a valid status on the specified date
    /// 
    /// 1 - the Taxpayer was in in the unified state register of taxpayers,
    /// but did not have a valid status on the specified date.
    /// 
    /// 2- the Taxpayer was in in the unified state register of taxpayers.
    /// 
    /// 3- the taxpayer was registered with the specified Taxpayer identification
    /// number in the unified state register of taxpayers, the reason code 
    /// of registration does not match or was not specified.
    /// 
    /// 4 - The taxpayer specified Taxpayer identification number 
    /// was not registered in the in the unified state register of taxpayers.
    /// 
    /// 5 - incorrect taxpayer identification number.
    /// 
    /// 6 - invalid number of characters of the identification number of the taxpayer.
    /// 
    /// 7 - invalid number of characters in the reason code of registration.
    /// 
    /// 8 - invalid characters in the identification number of the taxpayer.
    /// 
    /// 9 - invalid characters in reason code of registration.
    /// 
    /// 10 - not used
    /// 
    /// 11 - incorrect date format.
    /// 
    /// 12 - incorrect date (01.01.1991 earlier or later than the current date).
    /// 
    /// 
    /// For the individual entrepreneur:
    /// 
    /// 0 - the Taxpayer was in the unified state register of taxpayers
    /// and had a valid status on the specified date
    ///
    /// 1 - the Taxpayer was in in the unified state register of taxpayers,
    /// but did not have a valid status on the specified date.
    ///
    /// 2- the Taxpayer was in in the unified state register of taxpayers.
    /// 
    /// 3 - not used.
    /// 
    /// 4 - The taxpayer specified Taxpayer identification number 
    /// was not registered in the in the unified state register of taxpayers.
    /// 
    /// 5 - incorrect taxpayer identification number.
    ///    
    /// 6 - invalid number of characters of the identification number of the taxpayer.
    /// 
    /// 7 - not used.
    /// 
    /// 8 - invalid characters in the identification number of the taxpayer.
    /// 
    /// 9 - not used.
    /// 
    /// 10 - the reason code of registration should not be used when checking.
    /// 
    /// 11 - incorrect date format.
    /// 
    /// 12 - incorrect date (01.01.1991 earlier or later than the current date).
    /// 
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
