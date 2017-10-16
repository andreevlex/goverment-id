pub mod rpser;
pub mod http;
mod partner;

use std::result;

use self::rpser::xml::BuildElement;
use self::rpser::{ RpcError, Method };

use reqwest::{Error as ReqError};

use xmltree::Element;

use chrono::{DateTime, Utc};

pub use self::partner::Partner;

const V2_API_RPC_PATH: &'static str = "http://npchk.nalog.ru:80/FNSNDSCAWS_2";
const V2_API_REQUEST: &'static str = "http://ws.unisoft/FNSNDSCAWS2/Request";

pub trait FromElement {
    fn from_element(element: Element) -> Result<Self> where Self: Sized;
}

pub fn check_fns(partners: &Vec<Partner>) -> Result<rpser::Response> {
    let namespace = "req";   
    let mut nds_request2 = Method::new("NdsRequest2");
    for elem in partners.iter() {
        nds_request2 = nds_request2.with(Element::node(format!("{}:{}", namespace, "NP"))
            .with_attr( "INN", elem.inn.clone() )
            .with_attr( "KPP", elem.kpp.clone() )
            .with_attr( "DT", elem.dt.format("%d-%m-%Y").to_string() )
            );
    }
    
    call(nds_request2)
}

fn call(method: rpser::Method) -> Result<rpser::Response> {
       
        let envelope = method.as_xml(V2_API_REQUEST);
        //println!("[envelope xml] {:?}", envelope);

        let http_response = try!(http::soap_action(V2_API_RPC_PATH, &method.name, &envelope));

        println!("[response xml] {}", http_response.body);
        
        Ok(try!(rpser::Response::from_xml(&http_response.body)))
}

pub enum Error {
    TooManyRecords,
    Req(ReqError),
    Rpc(RpcError),
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

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {

}