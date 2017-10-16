
use Partner;

pub struct Nds_Response {
    DTActFL: String,
    DTActUL: String,
    partners: Vec<Partner>,
}

/*impl FromElement for Partner {
    fn from_element(element: Element) -> Result<Partner> {
        Ok(Partner {
            description: try!(element.get_at_path(&["description"])).text,
            home_page: try!(element.get_at_path(&["homePage"]).and_then(|e| e.as_long())),
            key: try!(element.get_at_path(&["key"]).and_then(|e| e.as_string())),
            name: try!(element.get_at_path(&["name"]).and_then(|e| e.as_string())),
            space_group: try!(element.get_at_path(&["name"])).text,
            space_type: try!(element.get_at_path(&["type"]).and_then(|e| e.as_string())),
            url: try!(element.get_at_path(&["url"]).and_then(|e| e.as_string())),
        })
    }
}*/
