use xmltree::Element;
use super::{error, NdsResponse, Partner, Result};
use super::rpser::xml::BuildElement;

use chrono::prelude::*;
use chrono::ParseResult;

use std::str::FromStr;

/// The trai to convert the server response xml to structure
pub trait FromElement {
    fn from_element(element: Element) -> Result<Self>
    where
        Self: Sized;
}

fn get_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    Utc.datetime_from_str(&format!("{} 00:00:00", value), "%d.%m.%Y %H:%M:%S")
}

impl<'a> FromElement for NdsResponse<'a> {
    fn from_element(element: Element) -> Result<NdsResponse<'a>> {
        let err_msg: String = element.get_attr("errMsg");
        if !err_msg.is_empty() {
            return Err(error::Error::FnsError(err_msg));
        }

        let mut rsp: NdsResponse = NdsResponse {
            dtact_fl: get_datetime(&element.get_attr("DTActFL"))?,
            dtact_ul: get_datetime(&element.get_attr("DTActUL"))?,
            partners: vec![],
        };

        for elm in element.children {
            rsp.partners.push(Partner::from_element(elm)?);
        }

        Ok(rsp)
    }
}

impl<'a> FromElement for Partner<'a> {
    fn from_element(element: Element) -> Result<Partner<'a>> {
        Ok(Partner {
            inn: element.get_attr("INN").into(),
            kpp: element.get_attr("KPP").into(),
            dt: get_datetime(&element.get_attr("DT"))?,
            state: i32::from_str(&element.get_attr("State"))?,
        })
    }
}
