use std::num;
use std::io;
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

pub fn check_fns(partners: Vec<Partner>) -> Result<NdsResponse> {
    let namespace = "req";

    if partners.len() > 10_000 {
        return Err(Error::TooManyRecords);
    }

    let mut nds_request2 = Method::new("NdsRequest2");
    for elem in partners {
        nds_request2 = nds_request2.with(
            Element::node(format!("{}:{}", namespace, "NP"))
                .with_attr("INN", elem.inn)
                .with_attr("KPP", elem.kpp)
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
    FnsError(String),
    ReqError(reqwest::Error),
    RpcError(rpser::RpcError),
    XmlError(rpser::xml::Error),
    ParseIntError(num::ParseIntError),
    ParseDateTimeError(chrono::ParseError),
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooManyRecords => write!(
                f,
                "В запросе не может быть больше 10000 элементов"
            ),
            Error::FnsError(ref err_msg) => write!(f, "{}", err_msg),
            Error::ReqError(ref e) => fmt::Display::fmt(e, f),
            Error::RpcError(ref e) => fmt::Display::fmt(e, f),
            Error::XmlError(ref e) => fmt::Display::fmt(e, f),
            Error::ParseIntError(ref e) => fmt::Display::fmt(e, f),
            Error::ParseDateTimeError(ref e) => fmt::Display::fmt(e, f),
            Error::IoError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TooManyRecords => {
                "В запросе не может быть больше 10000 элементов"
            }
            Error::FnsError(_) => {
                "Сервис сообщил об ошибке обработки запроса"
            }
            Error::ReqError(ref e) => e.description(),
            Error::RpcError(ref e) => e.description(),
            Error::XmlError(ref e) => e.description(),
            Error::ParseIntError(ref e) => e.description(),
            Error::ParseDateTimeError(ref e) => e.description(),
            Error::IoError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::TooManyRecords => None,
            Error::FnsError(_) => None,
            Error::ReqError(ref e) => e.cause(),
            Error::RpcError(ref e) => e.cause(),
            Error::XmlError(ref e) => e.cause(),
            Error::ParseIntError(ref e) => e.cause(),
            Error::ParseDateTimeError(ref e) => e.cause(),
            Error::IoError(ref e) => e.cause(),
        }
    }
}

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

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::IoError(other)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {}
