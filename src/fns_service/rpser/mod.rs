//! Remote procedule call implementation and serialization to XML.

pub mod xml;

use std::result;
use std::fmt;

use self::xml::BuildElement;
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
    pub fn as_xml(&self, api_url: &str) -> String {
        let namespace = "req";

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
        let mut element = Element::parse(&mut bytes).unwrap();

        if element.name != "Envelope" {
            return Err(RpcError::UnexpectedElement { tag: element.name });
        }
        element = try!(element.descend(&["Body"]));
        element = try!(element.descend_first());

        if element.name == "Fault" {
            return Err(RpcError::Fault {
                fault_code: try!(element.get_at_path(&["faultcode"]))
                    .text
                    .unwrap_or(String::new()),
                fault_string: try!(element.get_at_path(&["faultstring"]))
                    .text
                    .unwrap_or(String::new()),
                fault_detail: try!(element.get_at_path(&["detail"])),
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
#[derive(Debug, PartialEq)]
pub enum RpcError {
    Fault {
        fault_code: String,
        fault_string: String,
        fault_detail: Element,
    },
    XmlError { error: self::xml::Error },
    ExpectedElementText { tag: String },
    UnexpectedElement { tag: String },
    ElementWasEmpty { name: String },
    ElementNotFound { path: Vec<String> },
}

impl From<self::xml::Error> for RpcError {
    fn from(other: self::xml::Error) -> RpcError {
        RpcError::XmlError { error: other }
    }
}

pub type Result<T> = result::Result<T, RpcError>;


#[cfg(test)]
mod test {}
