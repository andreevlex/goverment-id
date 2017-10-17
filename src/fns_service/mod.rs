use std::num::ParseIntError;

pub mod rpser;
pub mod http;
pub mod transforms;

mod partner;
mod nds_response;

use std::result;

use self::rpser::xml::{BuildElement, Error as XmlError};
use self::rpser::{ RpcError, Method };

use reqwest::{Error as ReqError};

use xmltree::Element;

use chrono::{DateTime, Utc, ParseError };

pub use self::partner::Partner;
pub use self::nds_response::NdsResponse;

const V2_API_RPC_PATH: &'static str = "http://npchk.nalog.ru:80/FNSNDSCAWS_2";
const V2_API_REQUEST: &'static str = "http://ws.unisoft/FNSNDSCAWS2/Request";

pub trait FromElement {
    fn from_element(element: Element) -> Result<Self> where Self: Sized;
}

pub fn check_fns(partners: &Vec<Partner>) -> Result<NdsResponse> {
    let namespace = "req";   
    let mut nds_request2 = Method::new("NdsRequest2");
    for elem in partners.iter() {
        nds_request2 = nds_request2.with(Element::node(format!("{}:{}", namespace, "NP"))
            .with_attr( "INN", elem.inn.clone() )
            .with_attr( "KPP", elem.kpp.clone() )
            .with_attr( "DT", elem.dt.format("%d.%m.%Y").to_string() )
            );
    }
    
    let response = try!(call(nds_request2));
    
    Ok(try!(NdsResponse::from_element(response.body)))
}

fn call(method: rpser::Method) -> Result<rpser::Response> {
       
        let envelope = method.as_xml(V2_API_REQUEST);
        //println!("[envelope xml] {:?}", envelope);

        let http_response = try!(http::soap_action(V2_API_RPC_PATH, &method.name, &envelope));

        //println!("[response xml] {}", http_response.body);
        
        Ok(try!(rpser::Response::from_xml(&http_response.body)))
}

#[derive(Debug)]
pub enum Error {
    TooManyRecords,
    Req(ReqError),
    Rpc(RpcError),
    Xml(XmlError),
    /// Can't parse received element.
    ParseIntError(ParseIntError),
    /// Can't parse received element.
    ParseDateTimeError(ParseError),
}

impl From<ReqError> for Error {
    fn from(other: ReqError) -> Error {
        Error::Req(other)
    }
}

impl From<RpcError> for Error {
    fn from(other: RpcError) -> Error {
        Error::Rpc(other)
    }
}

impl From<XmlError> for Error {
    fn from(other: XmlError) -> Error {
        Error::Xml(other)
    }
}

impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Error {
        Error::ParseIntError(other)
    }
}

impl From<ParseError> for Error {
    fn from(other: ParseError) -> Error {
        Error::ParseDateTimeError(other)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {

}