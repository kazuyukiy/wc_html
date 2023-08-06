use regex::Regex;

pub fn page_json_blank() -> json::JsonValue {
    json::object! {
        // "syttem" :
        "system" : {
            // "version" : "0.0.1",
            // "version" : "0.0.2",
            "version" : "0.0.3",
        },

        "data" : {
            "page" : {
                "title" : "",
                "rev" : 0,
                "rev_speculation" : 0,
                "group_top" : false,
            },

            "navi" : [
                /*
                {"name0" : "href0"},
                {"name1" : "href1"}
                // change to
                ["name0" , "href0"],
                ["name0" , "href0"],

                 */
            ],

            "subsection" : {
                "id" : {
                    "id_next" : 1,
                    "id_notinuse" : []
                },

                "data" : {
                    "0" : {
                        "parent" : "",
                        "id" : "0",
                        "title" : "",
                        "href" : "",
                        "content" : [],
                        "child" : []
                    }
                    // ,

                    // "1" : {
                        // "parent" : "0",
                        // "id" : "1",
                        // "title" : "sample",
                        // "href" : "#sample",
                        // "content" : [ {"type" : "text", "value" : "sample"} ],
                        // "child" : []
                    // }

                },

            },

            "href" : {
                "relation" : {},
                "last" : {},
                "history" : {},
            },
        },
    }
} // end of fn page_json_blank

pub fn page_json_str_decode(json_str: &str) -> String {
    let json_str = String::from(json_str);

    // let json_str = json_str.replace("\\\"","\"");
    let json_str = json_str.replace("&lt;", "<");
    let json_str = json_str.replace("&gt;", ">");
    let json_str = json_str.replace("&amp;", "&");

    json_str
} // end of fn page_json_str_decode

pub fn page_json_value_from_str(json_str: &str) -> Option<json::JsonValue> {
    if json_str.len() == 0 {
        return None;
    }

    match json::parse(json_str) {
        // create PageJson from json value in self.page_dom()
        Ok(page_json_parse) => Some(page_json_parse),
        // create PageJson from blank json value
        Err(_) => {
            // json::JsonValue::new_object()
            // utility::page_json_blank()
            None
        }
    }
} // end of fn page_json_value_from_str

pub fn v_to_u32(jv: &json::JsonValue) -> Option<u32> {
    // let num_org_clone = jv.clone();
    // let num_org_string = num_org_clone.to_string();
    jv.as_u32()

    /*
    match jv.as_str() {
        Some(v) => { return v.as_u32(); },
        None => { return None; },
    }
     */

    /*
    let num_org_string = jv.clone().to_string();
    let num_org_result = u32::from_str_radix(&num_org_string, 10);
    match num_org_result {
        Ok(i) => Ok(i),
        _ => Err(()),
    }
     */
}

pub fn n_to_int(jv_n: &json::JsonValue) -> Result<u32, ()> {
    let num_org_clone = jv_n.clone();
    let num_org_string = num_org_clone.to_string();
    let num_org_result = u32::from_str_radix(&num_org_string, 10);
    match num_org_result {
        Ok(i) => Ok(i),
        _ => Err(()),
    }
} // end of n_to_int

pub fn num_plus(jv_n: &mut json::JsonValue, num_arg: u32) -> u32 {
    let num_re = n_to_int(jv_n);
    let num_org;
    match num_re {
        Ok(i) => num_org = i,
        Err(_) => return num_arg,
    }

    let num_new = num_org + num_arg;
    *jv_n = num_new.into();

    num_new
} // end of fn num_plus

// Find page_json value in text form <span> or <script> element
fn fined_page_json_str(str: &str) -> Option<&str> {
    if let Some(s) = find_page_json_str_in_span(str) {
        return Some(s);
    }
    if let Some(s) = find_page_json_str_in_script(str) {
        return Some(s);
    }

    None
} // end of fn fined_page_json_str

// Find <span id="page_json_str" style="display: none">{...}</span>
// Return Some({...})
// for system 0.0.2 or later.
fn find_page_json_str_in_span(str: &str) -> Option<&str> {
    // ...<span id="page_json_str" ... >{...}</span>...
    // <span id="page_json_str" ... >
    let re = Regex::new(r#"<(?i)span[^>]*id\s*=\s*"page_json_str"[^>]*>+\s*[^{]*"#).unwrap();
    let mat = re.find(&str);

    if let None = mat {
        return None;
    }
    let start = mat.unwrap().end();

    // </span
    let re = Regex::new(r#"</(?i)span\s*"#).unwrap();
    let mat = re.find_at(&str, start);
    if let None = mat {
        return None;
    }
    let end = mat.unwrap().start();
    // println!("match: {}", &str[start..end]);
    Some(&str[start..end])
} // end of fn find_page_json_str_in_span

// find <script type="text/javascript" class="page_json">let page_json = {...}</script>
// Return Some({...})
// for system 0.0.1
fn find_page_json_str_in_script(str: &str) -> Option<&str> {
    // <script type="text/javascript" class="page_json"> let page_json=
    let re =
        Regex::new(r#"<(?i)script[^>]*class\s*=\s*"page_json"[^>]*>+\s*let\s+page_json\s*=[^{]*"#)
            .unwrap();
    let mat = re.find(&str);
    if let None = mat {
        return None;
    }
    let start = mat.unwrap().end();
    // </script
    let re = Regex::new(r#"\s*</(?i)script\s*"#).unwrap();
    let mat = re.find_at(&str, start);
    if let None = mat {
        return None;
    }
    let end = mat.unwrap().start();
    Some(&str[start..end])
    // println!("script match: {}", &str[start..end]);
} // end of fn find_page_json_str_in_script

// Find json data from str.
// Convert json str to json value.
pub fn page_json_value_from_content(content: &str) -> Option<json::JsonValue> {
    let page_json_str = fined_page_json_str(content);

    // page_json in str is not found in the page.
    if let None = page_json_str {
        return None;
    }

    // Some charactors are encoded to entity reference in html pages .
    let page_json_str = page_json_str_decode(&page_json_str.unwrap());

    // page_json_value_from_str(&page_json_str.unwrap())
    page_json_value_from_str(&page_json_str)
} // end of fn page_json_value_from_content
