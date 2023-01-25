// use crate::page5;
// use page;
// use super::page;
use crate::page;

use crate::page::page_json_utility;
// pub use crate::page_json_utility;

// href is a reference to the target in some page .
// the location of the target might be changed in some reason sometimes .
// To avoid to loose references to its targets, make an intermediate reference
// in self.page.page_json().value()["data"]["href_reference"] of a page
// The intermediate reference is not in the page where the target exists,
// but in some another page that is considerd not so often moved its location .
// ex. a top page of a group pages has an intermediate reference
// and there are some sub pages .
// Even some changes happen on sub pages ,
// the changes are applied to the intermediate reference
// so if you refer the intermediate reference you can reach to the destinations.

// caller_url : url of page where href link is set
// and looking for its destination from
//
//  req_url : a link location data set in caller_url page
//
pub struct HrefInspec<'a> {
    // pub struct HrefInspec {
    caller_page: page::Page,
    req_url_str: &'a str,
    req_page: page::Page,
    imd_page: Option<page::Page>,
    hist: Option<Vec<String>>,
} // end of struct HrefInspec

impl HrefInspec<'_> {
    // impl HrefInspec {
    // println!("href_inspec impl HrefInspec");

    pub fn from<'a>(caller_url: &url::Url, req_url_str: &'a str) -> Result<HrefInspec<'a>, ()> {
        let caller_page = page::Page::from(&caller_url);

        // req_url_str
        // Keep req_url_str as it is,
        // caller_page.url may have same data,
        // But to update req_url_str to new one,
        // the exact string is required to replace it .

        let req_url = match caller_page.url.join(&req_url_str) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        let req_page = page::Page::from(&req_url);

        let href_inspec = HrefInspec {
            caller_page,
            req_url_str,
            req_page,
            imd_page: None,
            hist: None,
        };

        Ok(href_inspec)
    } // end of fn from

    pub fn href_req_handle(&mut self) -> Option<Vec<u8>> {
        // println!("href_inspec5 impl HrefInspec fn href_req_handle");

        // ATTENTION
        println!("ATTENTION href_inspec5 impl HrefInspec fn href_req_handle about self.imd_page is not corded completely");

        let (dest, url_valid) = self.dest();

        // println!("href_inspec5 impl HrefInspec fn href_req_handle dest: {:?}", dest);

        let mut jv = json::object! {};

        if let Some(dest) = dest {
            // jv["dest"] = dest[url::Position::AfterHost..].into();
            jv["dest"] = dest.path().into();
        }

        self.href_update(url_valid);

        println!(
            "href_inspec5 impl HrefInspec fn href_req_handle jv: {:?}",
            &jv
        );

        Some(jv.to_string().into_bytes())
    } // end of fn href_req_handle

    // Return (destination_url, url_valid)
    // destination_url : url to response to client
    // url_valid : url to be updated to
    fn dest(&mut self) -> (Option<url::Url>, Option<url::Url>) {
        // fn dest(&mut self) -> (Option<&url::Url>, Option<&url::Url>) {
        println!("href_inspec5 impl HrefInspec fn dest");

        if self.hash_none() {
            let url_valid = match self.imd_page.as_ref() {
                Some(imd_page) => &imd_page.url,
                // if no imd_page, self.caller_page.url itself is valid href
                None => &self.caller_page.url,
            };

            return (Some(self.req_page.url.clone()), Some(url_valid.clone()));
        }

        if let Err(_) = self.hist() {
            // DBG
            // println!("href_inspec5 impl HrefInspec fn dest url duplicated");

            return (None, None);
        }

        println!("href_inspec5 impl HrefInspec fn dest cp1");

        // hash of self.req_pate.url is found in a subseciton of self.req_page
        // means &self.imd_page.url is valid href
        if self.hash_in_page_subsection() {
            let url_valid = match self.imd_page.as_ref() {
                Some(imd_page) => &imd_page.url,
                // if no imd_page, self.caller_page.url itself is valid href
                None => &self.caller_page.url,
            };

            return (Some(self.req_page.url.clone()), Some(url_valid.clone()));
        }

        println!("href_inspec5 impl HrefInspec fn dest cp2");

        if let Some(url_ref_new) = self.in_hash_reference() {
            if let Ok(v) = HrefInspec::from(&self.req_page.url, &url_ref_new) {
                let mut href_inspec = v;
                return href_inspec.dest();
            }
        }

        // if hash == "#top"
        // and it is not found in in_hash_reference
        // it shoul be linkded to req_page .
        // if #top found in_hash_reference,
        // it means req_page was moved to some page recorded in in_hash_reference
        // otherwise it shoud be link to top of req_page.
        let hash = &self.req_page.url[url::Position::AfterPath..];
        if hash == "#top" {
            let url_valid = match self.imd_page.as_ref() {
                Some(imd_page) => &imd_page.url,
                // if no imd_page, self.caller_page.url itself is valid href
                None => &self.caller_page.url,
            };
            return (Some(self.req_page.url.clone()), Some(url_valid.clone()));
        }

        // println!("href_inspec5 impl HrefInspec fn dest hash: {}", hash);

        // temp
        (None, None)
    } // end of fn dest

    fn hash_none(&self) -> bool {
        let hash = &self.req_page.url[url::Position::AfterPath..];
        if hash.len() == 0 {
            return true;
        }

        false
    } // end of fn hash_none

    // If uri already in self.hist, return Err
    // to avoid to loop same url recursively
    // Push uri into self.hist to see wether same uri comes later
    fn hist(&mut self) -> Result<(), ()> {
        let req_url_str = self.req_page.url.to_string();

        // let url = self.req_url.as_str();
        // if url.len() == 0 { return Err(()); }

        if req_url_str.len() == 0 {
            return Err(());
        }

        // self.hist initialize
        if let None = self.hist {
            self.hist.replace(vec![]);
        }

        // let url_string = url.to_string();
        // if self.hist.as_ref().unwrap().contains(&url.to_string()) {
        // if self.hist.as_ref().unwrap().contains(&url_string) {
        if self.hist.as_ref().unwrap().contains(&req_url_str) {
            return Err(());
        }

        self.hist.as_mut().unwrap().push(req_url_str);

        Ok(())
    } // end of fn hist

    // See if hash value is found in subsection href of req_url page .
    // req_url: /abc/def.html#ghi
    // subsection["href"] == "ghi"
    // in any of req_page_json["data"]["subsection"]["data"];
    fn hash_in_page_subsection(&mut self) -> bool {
        // println!("href_inspec5 impl HrefInspec fn hash_in_page_subsection");

        // let hash = &self.req_url[url::Position::AfterPath..];
        let hash = &self.req_page.url[url::Position::AfterPath..];

        // // the self.req_page is wc style and the hash_part is #top
        // // #top can not be found in page_json data.
        // // But it is top of the page.
        // if self.req_page.is_wc_page() && hash == "#top" { return true; }

        // let page_json = self.req_page.page_json().value();
        let page_json = match self.req_page.page_json.as_ref() {
            Some(v) => v.value(),
            None => return false,
        };

        let subsections = &page_json["data"]["subsection"]["data"];

        // Scan each subsection data to find a subsection that has hash value .
        // It look at only subsection's href value .
        // It does not look insode of the contents .
        // if href matches hash_part, return true .
        let hash_jv = json::JsonValue::from(hash);
        match subsections {
            json::JsonValue::Object(o) => {
                for (_id, subsection) in o.iter() {
                    // if subsection["href"] == json::JsonValue::from(hash) {
                    if subsection["href"] == hash_jv {
                        // ATTENTION !!
                        println!("href_inspec5 impl HrefInspec fn hash_in_page_subsection href : Consider to avoid href is empty");

                        // println!("href_inspec5 impl HrefInspec fn hash_in_page_subsection href match: {:?}", &hash_jv);

                        return true;
                    }
                }
            }
            _ => {
                return false;
            }
        }

        false
    } // end of fn hash_in_page_subsection

    // pub fn in_hash_reference(&mut self) -> Option<String> {
    pub fn in_hash_reference(&self) -> Option<&str> {
        let hash = &self.req_page.url[url::Position::AfterPath..];

        let page_json = match self.req_page.page_json.as_ref() {
            Some(v) => v.value(),
            None => return None,
        };
        let href_ref = &page_json["data"]["href_reference"];

        match href_ref {
            json::JsonValue::Object(o) => {
                for (key, value) in o.iter() {
                    if key == hash {
                        // return Some(value.to_string());
                        // return Some(&value);
                        // return Some(&value.as_str());
                        match &value.as_str() {
                            Some(v) => return Some(v),
                            None => (),
                        }
                    }
                }
            }
            _ => (), // {},
        }

        None
    } // end of fn in_hash_reference

    // If more applopriate href found, replace it
    fn href_update(&mut self, url_valid: Option<url::Url>) {
        let url_valid = match url_valid {
            Some(v) => v,
            None => return,
        };

        // No change href
        if self.req_page.url[url::Position::AfterHost..] == url_valid[url::Position::AfterHost..] {
            return;
        }

        // Get page_json in string
        // let page_json_str = page.page_json().value().to_string();
        let page_json_str = match self.caller_page.page_json.as_ref() {
            Some(v) => v.value().to_string(),
            None => return,
        };

        let page_json_str = page_json_utility::page_json_str_decode(&page_json_str);

        // replace from
        // href="abc/def.html#ghi"
        let from = r#"href=""#.to_owned()
			+
		// &self.req_url[url::Position::AfterHost..]
			&self.req_url_str
			+
			r#"""#;
        // href=\"abc/def.html#ghi\"
        let from = from.replace(r#"""#, r#"\""#);

        if page_json_str.contains(&from) == false {
            return;
        }

        // relative, if not, absolute
        let to_url = match self.caller_page.url.make_relative(&url_valid) {
            // relative
            Some(url) => url,
            // absolute
            None => url_valid[url::Position::AfterHost..].to_string(),
        };

        // replace to
        let to = r#"href=""#.to_owned() + &to_url + r#"""#;
        let to = to.replace(r#"""#, r#"\""#);

        // Replace from to to
        let page_json_str = page_json_str.replace(&from, &to);

        // Convert string to JaonValue
        let page_json = match page_json_utility::page_json_value_from_str(&page_json_str) {
            Some(v) => v,
            None => return,
        };

        if let Err(_) = self.caller_page.page_json_replace(page_json) {
            return;
        }

        self.caller_page.page_current_save();
        self.caller_page.page_json_update_save();
    } // end of fn href_update
} // end of impl HrefInspec
