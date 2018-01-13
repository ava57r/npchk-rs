extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate xml;
extern crate xmltree;

mod rpser;
mod http;
mod transforms;
pub mod models;
pub mod error;

use std::result;

use self::rpser::xml::BuildElement;
use self::rpser::Method;

use xmltree::Element;

pub use self::models::partner::Partner;
pub use self::models::nds_response::NdsResponse;

pub use self::transforms::FromElement;

const V2_API_RPC_PATH: &'static str = "http://npchk.nalog.ru:80/FNSNDSCAWS_2";
const V2_API_REQUEST: &'static str = "http://ws.unisoft/FNSNDSCAWS2/Request";
const V2_API_NAMESPACE: &'static str = "req";

/// Проверяет контрагентов с помощью сервиса http://npchk.nalog.ru/
pub fn check_fns(partners: Vec<Partner>) -> Result<NdsResponse> {
    if partners.len() > 10_000 {
        return Err(error::Error::TooManyRecords);
    }

    let mut nds_request2 = Method::new("NdsRequest2");
    for elem in partners {
        nds_request2 = nds_request2.with(
            Element::node(format!("{}:{}", V2_API_NAMESPACE, "NP"))
                .with_attr("INN", elem.inn)
                .with_attr("KPP", elem.kpp)
                .with_attr("DT", elem.dt.format("%d.%m.%Y").to_string()),
        );
    }

    let response = call(nds_request2)?;

    Ok(NdsResponse::from_element(response.body)?)
}

/// Проверяет 1 - го контрагента с помощью сервиса http://npchk.nalog.ru/
pub fn check_fns_partner(p: Partner) -> Result<NdsResponse> {
    let mut partners: Vec<Partner> = vec![];
    partners.push(p);

    check_fns(partners)
}

/// Вызывает удаленную процедуру через протокол `SOAP`
fn call(method: rpser::Method) -> Result<rpser::Response> {
    let envelope = method.as_xml(V2_API_REQUEST, V2_API_NAMESPACE);

    let http_response = http::soap_action(V2_API_RPC_PATH, &method.name, &envelope)?;

    Ok(rpser::Response::from_xml(&http_response.body)?)
}

pub type Result<T> = result::Result<T, error::Error>;

#[cfg(test)]
mod tests {}