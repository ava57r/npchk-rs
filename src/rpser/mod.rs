//! Remote procedule call implementation and serialization to XML.

pub mod xml;

use std::result;
use std::error;
use std::fmt;

use self::xml::BuildElement;
use xmltree;
use xmltree::Element;

/// XML method representation.
#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub args: Vec<Element>,
}

impl Method {
    /// Create new method with name.
    pub fn new(name: &str) -> Method {
        Method {
            name: name.into(),
            args: vec![],
        }
    }

    /// Add argument to method.
    ///
    /// The `arg` is XML Element.
    pub fn with(mut self, arg: Element) -> Self {
        self.args.push(arg);
        self
    }

    /// Convert method to full XML envelope.
    pub fn as_xml(&self, api_url: &str, namespace: &str) -> String {
        let envelope = Element::node("soapenv:Envelope")
            .with_attr("xmlns:soapenv", "http://schemas.xmlsoap.org/soap/envelope/")
            .with_attr(format!("xmlns:{}", namespace), api_url)
            .with_children(vec![
                Element::node("soapenv:Header"),
                Element::node("soapenv:Body").with_child(
                    Element::node(format!("{}:{}", namespace, self.name))
                        .with_children_from_iter(self.args.iter()),
                ),
            ]);

        envelope.to_string()
    }
}

/// XML response representation.
#[derive(Debug)]
pub struct Response {
    pub body: Element,
}

impl Response {
    /// Parse response from XML.
    pub fn from_xml(xml: &str) -> Result<Response> {
        let mut bytes = xml.as_bytes();
        let mut element = Element::parse(&mut bytes)?;

        if element.name != "Envelope" {
            return Err(RpcError::UnexpectedElement { tag: element.name });
        }
        element = element.descend(&["Body"])?;
        element = element.descend_first()?;

        if element.name == "Fault" {
            return Err(RpcError::Fault {
                fault_code: element
                    .get_at_path(&["faultcode"])?
                    .text
                    .unwrap_or(String::new()),
                fault_string: element
                    .get_at_path(&["faultstring"])?
                    .text
                    .unwrap_or(String::new()),
                fault_detail: element.get_at_path(&["detail"])?,
            });
        }

        Ok(Response { body: element })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.args)
    }
}

/// Method parsing / response error.
#[derive(Debug)]
pub enum RpcError {
    Fault {
        fault_code: String,
        fault_string: String,
        fault_detail: Element,
    },
    XmlError { error: self::xml::Error },
    XmlTreeError { error: xmltree::ParseError },
    ExpectedElementText { tag: String },
    UnexpectedElement { tag: String },
    ElementWasEmpty { name: String },
    ElementNotFound { path: Vec<String> },
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RpcError::Fault {
                ref fault_code,
                ref fault_string,
                ref fault_detail,
            } => write!(
                f,
                "Fault: {}\n{}\n{:?}",
                fault_code,
                fault_string,
                fault_detail
            ),
            RpcError::XmlError { error: ref e } => fmt::Display::fmt(e, f),
            RpcError::ExpectedElementText { ref tag } => write!(f, "Expected element text {}", tag),
            RpcError::XmlTreeError { error: ref e } => fmt::Display::fmt(e, f),
            RpcError::UnexpectedElement { ref tag } => write!(f, "Unexpected element {}", tag),
            RpcError::ElementWasEmpty { ref name } => write!(f, "Element was empty {}", name),
            RpcError::ElementNotFound { ref path } => write!(f, "Element not found\n {:?}", path),
        }
    }
}

impl error::Error for RpcError {
    fn description(&self) -> &str {
        match *self {
            RpcError::Fault {
                fault_code: _,
                fault_string: _,
                fault_detail: _,
            } => "Fault remote procedure call",
            RpcError::XmlError { error: ref e } => e.description(),
            RpcError::XmlTreeError { error: ref e } => e.description(),
            RpcError::ExpectedElementText { tag: _ } => "Expected element text",
            RpcError::UnexpectedElement { tag: _ } => "Unexpected element {}",
            RpcError::ElementWasEmpty { name: _ } => "Element was empty",
            RpcError::ElementNotFound { path: _ } => "Element not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RpcError::Fault {
                fault_code: _,
                fault_string: _,
                fault_detail: _,
            } => None,
            RpcError::XmlError { error: ref e } => e.cause(),
            RpcError::XmlTreeError { error: ref e } => e.cause(),
            RpcError::ExpectedElementText { tag: _ } => None,
            RpcError::UnexpectedElement { tag: _ } => None,
            RpcError::ElementWasEmpty { name: _ } => None,
            RpcError::ElementNotFound { path: _ } => None,
        }
    }
}

impl From<self::xml::Error> for RpcError {
    fn from(other: self::xml::Error) -> RpcError {
        RpcError::XmlError { error: other }
    }
}

impl From<xmltree::ParseError> for RpcError {
    fn from(other: xmltree::ParseError) -> RpcError {
        RpcError::XmlTreeError { error: other }
    }
}

pub type Result<T> = result::Result<T, RpcError>;


#[cfg(test)]
mod test {}
