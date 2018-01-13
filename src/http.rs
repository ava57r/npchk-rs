// get from https://github.com/Nercury/confluence-rs
//! HTTP helpers.

use std::io::Read;

use reqwest::{Client, StatusCode};

use hyper::header::ContentType;
use hyper::mime;

header! { (SoapAction, "SOAPAction") => [String] }

/// Simplified HTTP response representation.
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

/// Perform a GET request to specified URL.
pub fn get(url: &str) -> super::Result<Response> {
    let client = Client::new()?;
    let mut response = client.get(url)?.send()?;

    let mut body = String::new();
    response.read_to_string(&mut body)?;

    Ok(Response {
        status: response.status(),
        body: body,
    })
}

/// Perform a SOAP action to specified URL.
pub fn soap_action(url: &str, action: &str, xml: &str) -> super::Result<Response> {
    let client = Client::new()?;
    let mut response = client
        .post(url)?
        .header(ContentType(mime::TEXT_XML))
        .header(SoapAction(action.into()))
        .body(xml.to_string())
        .send()?;

    let mut body = String::new();
    response.read_to_string(&mut body)?;

    Ok(Response {
        status: response.status(),
        body: body,
    })
}
