use xmltree::Element;
use fns_service::{FromElement, Result, Partner, NdsResponse};
use fns_service::rpser::xml::BuildElement;
use chrono::prelude::*;
use chrono::ParseResult;

use std::str::FromStr;

fn get_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    Utc.datetime_from_str(&format!("{} 00:00:00", value), "%d.%m.%Y %H:%M:%S")
}

impl FromElement for NdsResponse {
    fn from_element(element: Element) -> Result<NdsResponse> {
        let mut rsp: NdsResponse = NdsResponse {
            dtact_fl: try!(get_datetime(&element.get_attr("DTActFL"))),
            dtact_ul: try!(get_datetime(&element.get_attr("DTActUL"))),
            partners: vec![],
            };
        
        for elm in element.children {
            rsp.partners.push(try!(Partner::from_element(elm)));
        }

        Ok(rsp)
    }
}

impl FromElement for Partner {
    fn from_element(element: Element) -> Result<Partner> {
        Ok(Partner {
            inn: element.get_attr("INN"),
            kpp: element.get_attr("KPP"),
            dt: try!(get_datetime(&element.get_attr("DT"))),
            state: try!(i32::from_str(&element.get_attr("State"))),
        }        
        )
    }
}