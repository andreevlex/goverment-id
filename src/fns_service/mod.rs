use std::num;
use reqwest;
use chrono;

pub mod rpser;
pub mod http;

mod partner;
mod nds_response;
mod transforms;

use std::result;
use std::error;
use std::fmt;

use self::rpser::xml::BuildElement;
use self::rpser::Method;

use xmltree::Element;

pub use self::partner::Partner;
pub use self::nds_response::NdsResponse;

pub use self::transforms::FromElement;

const V2_API_RPC_PATH: &'static str = "http://npchk.nalog.ru:80/FNSNDSCAWS_2";
const V2_API_REQUEST: &'static str = "http://ws.unisoft/FNSNDSCAWS2/Request";

pub fn check_fns(partners: &Vec<Partner>) -> Result<NdsResponse> {
    let namespace = "req";
    let mut nds_request2 = Method::new("NdsRequest2");
    for elem in partners.iter() {
        nds_request2 = nds_request2.with(
            Element::node(format!("{}:{}", namespace, "NP"))
                .with_attr("INN", elem.inn.clone())
                .with_attr("KPP", elem.kpp.clone())
                .with_attr("DT", elem.dt.format("%d.%m.%Y").to_string()),
        );
    }

    let response = call(nds_request2)?;

    Ok(NdsResponse::from_element(response.body)?)
}

fn call(method: rpser::Method) -> Result<rpser::Response> {
    let envelope = method.as_xml(V2_API_REQUEST);
    
    let http_response = http::soap_action(V2_API_RPC_PATH, &method.name, &envelope)?;

    Ok(rpser::Response::from_xml(&http_response.body)?)
}

#[derive(Debug)]
pub enum Error {
    TooManyRecords,
    ReqError(reqwest::Error),
    RpcError(rpser::RpcError),
    XmlError(rpser::xml::Error),
    ParseIntError(num::ParseIntError),
    ParseDateTimeError(chrono::ParseError),
}

/*impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooManyRecords => write!(f, "В запросе не может быть больше 10000 элементов"),
            Error::ReqError(ref e) => fmt::Display::fmt(e, f),
            Error::RpcError(ref e) => fmt::Display::fmt(e, f),

        }
    }
}

impl error::Error for Error {

}*/

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Error {
        Error::ReqError(other)
    }
}

impl From<rpser::RpcError> for Error {
    fn from(other: rpser::RpcError) -> Error {
        Error::RpcError(other)
    }
}

impl From<rpser::xml::Error> for Error {
    fn from(other: rpser::xml::Error) -> Error {
        Error::XmlError(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Error {
        Error::ParseIntError(other)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(other: chrono::ParseError) -> Error {
        Error::ParseDateTimeError(other)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {}
