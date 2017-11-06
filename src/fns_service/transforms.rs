use xmltree::Element;
use super::{NdsResponse, Partner, Result, Error};
use super::rpser::xml::BuildElement;

use chrono::prelude::*;
use chrono::ParseResult;

use std::str::FromStr;

pub trait FromElement {
    fn from_element(element: Element) -> Result<Self>
    where
        Self: Sized;
}

fn get_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    Utc.datetime_from_str(&format!("{} 00:00:00", value), "%d.%m.%Y %H:%M:%S")
}

impl FromElement for NdsResponse {
    fn from_element(element: Element) -> Result<NdsResponse> {
        let err_msg: String = element.get_attr("errMsg");
        if !err_msg.is_empty() {
            return Err(Error::FnsError(err_msg));
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

impl FromElement for Partner {
    fn from_element(element: Element) -> Result<Partner> {
        Ok(Partner {
            inn: element.get_attr("INN"),
            kpp: element.get_attr("KPP"),
            dt: get_datetime(&element.get_attr("DT"))?,
            state: i32::from_str(&element.get_attr("State"))?,
        })
    }
}
