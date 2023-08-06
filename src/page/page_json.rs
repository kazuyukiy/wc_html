// pub use crate::page_json_utility as utility;
use super::page_json_utility as utility;

pub struct PageJson {
    pub page_json: json::JsonValue,
    rev: Option<u32>,
    rev_next: Option<u32>,
} // end of struct PageJson

impl PageJson {
    pub fn new(page_json: json::JsonValue) -> PageJson {
        PageJson {
            page_json,
            rev: None,
            rev_next: None,
        }
    } // end of fn new

    pub fn from_str(json_str: &str) -> PageJson {
        let page_json = match utility::page_json_value_from_str(json_str) {
            Some(v) => v,
            None => utility::page_json_blank(),
        };

        Self::new(page_json)
    } // end of fn from_str

    pub fn value(&self) -> &json::JsonValue {
        &self.page_json
    } // end of fn value

    pub fn value_mut(&mut self) -> &mut json::JsonValue {
        &mut self.page_json
    } // end of fn value_mut

    pub fn value_replace(&mut self, page_json: json::JsonValue) {
        self.page_json = page_json;
    } // end of fn value_replace

    // The original rev
    // Return value in self.page_json["data"]["page"]["rev"]
    // if no value, defined rev value as rev_default ( 0 )
    // Return the original rev number
    // even self.page_json["data"]["page"]["rev"] is updated to next value
    pub fn rev(&mut self) -> u32 {
        // Case self.rev is None
        // self.rev is not initialized
        // set rev value and put it into self.rev
        if let None = self.rev {
            let rev_default = 0;
            let rev;
            if self.page_json["data"]["page"]["rev"].is_null() {
                rev = rev_default;
            } else {
                let rev_op = self.page_json["data"]["page"]["rev"].as_u32();
                match rev_op {
                    Some(r) => rev = r,
                    None => rev = rev_default,
                }
            }

            self.rev.replace(rev);
        }

        *self.rev.as_ref().unwrap()
    } // end of fn rev

    // Next rev value to be used.
    // This value can be only next number of self.rev
    // No further proceeded number
    pub fn rev_next(&mut self) -> u32 {
        // self.rev_next is None: not initialized
        if let None = self.rev_next {
            // self.page_json["data"]["page"]["rev"]: josn::JsonValue::Short
            self.page_json["data"]["page"]["rev"] = (self.rev() + 1).into();
            let rev_next = self.page_json["data"]["page"]["rev"].as_u32().unwrap();
            self.rev_next.replace(rev_next);
        }

        self.rev_next.unwrap()
    } // end of fn rev_next
} // end of impl PageJson
