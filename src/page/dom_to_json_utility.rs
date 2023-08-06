// use std::rc::Rc;
// use std::cell::RefCell;

pub fn json_value_to_u32(jv: &mut json::JsonValue) -> u32 {
    let mut v: u32 = 0;
    let jv_clone = jv.clone();
    let jv_string = jv_clone.to_string();
    let jv_result = u32::from_str_radix(&jv_string, 10);
    match jv_result {
        Ok(i) => {
            v = i;
        }
        _ => (),
    }

    v
} // end of fn json_value_to_u32

fn json_id_next(page_json: &mut json::JsonValue) -> u32 {
    // fn json_id_next(page_json: &json::JsonValue) -> u32 {
    // let id = json_id_next(&mut page_json);

    // id_notinuse
    // let json_id = &mut page_json["data"]["subsection"]["id"];
    // let json_id = &page_json["data"]["subsection"]["id"];
    let json_id = &mut page_json["data"]["subsection"]["id"];
    if 0 < json_id["id_notinuse"].len() {
        let id_string = json_id["id_notinuse"].array_remove(0).to_string();
        let id_result = u32::from_str_radix(&id_string, 10);
        match id_result {
            Ok(i) => {
                return i;
            }
            _ => (),
        }
    }

    // id_next
    let mut id: u32 = 0;
    let id_clone = json_id["id_next"].clone();
    let id_string = id_clone.to_string();
    let id_result = u32::from_str_radix(&id_string, 10);
    match id_result {
        Ok(i) => {
            id = i;
        }
        _ => (),
    }

    json_id["id_next"] = (id + 1).into();

    id
} // end of fn json_id_next

// Returns the json by id.
fn ss_json_id<'a>(
    page_json: &'a mut json::JsonValue,
    id: &u32,
) -> Result<&'a mut json::JsonValue, ()> {
    if page_json["data"]["subsection"]["data"][id.to_string()].is_null() {
        return Err(());
    }
    Ok(&mut page_json["data"]["subsection"]["data"][id.to_string()])
} // fn ss_json_id

// Return the id by href.
fn ss_id_href(page_json: &json::JsonValue, href: &str) -> Result<String, ()> {
    let subsection_obj = &page_json["data"]["subsection"]["data"];
    // enum `JsonValue`

    let ss_id_match: String;
    match subsection_obj {
        json::JsonValue::Object(o) => {
            for (ss_id, ss_json) in o.iter() {
                // println!("ss_json_href ss_id: {}", ss_id);
                match ss_json {
                    json::JsonValue::Object(o2) => {
                        for (key, value) in o2.iter() {
                            if key == "href" {
                                if value == href {
                                    ss_id_match = ss_id.to_string();
                                    return Ok(ss_id_match);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    Err(())
} // end of fn ss_id_href

pub fn ss_json_new<'a>(
    page_json: &'a mut json::JsonValue,
    href: &str,
    pid: u32,
) -> &'a mut json::JsonValue {
    let id_res = ss_id_href(page_json, href);
    match id_res {
        // ss_json exists with href
        Ok(id) => {
            return &mut page_json["data"]["subsection"]["data"][id];
        }
        _ => (),
    }

    let id = json_id_next(page_json);

    // let pid_u: u32 = *pid.borrow();
    let mut ss_json_parent = &mut json::object! {};
    // let ss_json_parent_op = ss_json_id(page_json, &pid_u);
    let ss_json_parent_op = ss_json_id(page_json, &pid);
    match ss_json_parent_op {
        Ok(s) => ss_json_parent = s,
        _ => (),
    }
    let _r = ss_json_parent["child"].push(id);

    page_json["data"]["subsection"]["data"][id.to_string()] = json::object! {};
    let ss_json = &mut page_json["data"]["subsection"]["data"][id.to_string()];
    ss_json["id"] = id.into();
    // ss_json["parent"] = pid_u.into();
    ss_json["parent"] = pid.into();
    ss_json["href"] = href.into();
    ss_json["child"] = json::JsonValue::new_array();
    ss_json["content"] = json::JsonValue::new_array();

    ss_json
} // end of fn ss_json_new

pub fn ss_json_href<'a>(
    page_json: &'a mut json::JsonValue,
    href: &str,
) -> Option<&'a mut json::JsonValue> {
    // pub fn ss_json_href<'a>(page_json: &'a mut json::JsonValue, href: &str) -> Result<&'a mut json::JsonValue, ()> {
    // pub fn ss_json_href<'a>(page_json: &'a json::JsonValue, href: &str) -> Result<&'a mut json::JsonValue, ()> {

    // let mut id_op: Option<&str> = None;
    let mut id_op = None;

    {
        // let subsection_json = &mut page_json["data"]["subsection"]["data"];
        // let subsection_json = &page_json["data"]["subsection"]["data"];
        let subsection_json = &page_json["data"]["subsection"]["data"];
        // if subsection_json.is_null() { return Err(()); }
        if subsection_json.is_null() {
            return None;
        }
        // subsection_json : { "0" : {}, "1" : {},}
        if let json::JsonValue::Object(o) = subsection_json {
            // for ss_json_tup in o.iter_mut() {
            'l: for ss_json_tup in o.iter() {
                // ss_json_tup: ( 0: name, 1: value)
                if let json::JsonValue::Object(p) = ss_json_tup.1 {
                    // for tup2 in p.iter_mut() {
                    for tup2 in p.iter() {
                        if tup2.0 == "href" && tup2.1 == href {
                            // println!("dom_to_json_urility fn ss_json_href match id: {}", &ss_json_tup.0);
                            id_op = Some(String::from(ss_json_tup.0));
                            // id_op = Some(ss_json_tup.0);
                            // id_op = Some(ss_json_tup.0.clone());
                            break 'l;
                        }
                    }
                }
            }
        }
    }

    if let Some(id) = id_op {
        return Some(&mut page_json["data"]["subsection"]["data"][&id]);
    }

    // Err(())
    None

    /*
    match subsection_json {
        json::JsonValue::Object(o) => {
            for ss_json_tup in o.iter_mut() {
                // ss_json_tup: ( 0: name, 1: value)
                println!("dom_to_json_utility fn ss_json_href ss_json_tup.0 : {}", &ss_json_tup.0);
                if ss_json_tup.0 == "href" {
                    let ss_json = ss_json_tup.1;
                    if ss_json["href"] == href {
                        return Ok(ss_json);
                    }
                }
            }
        },
        _ => (),
    }
    Err(())
     */
} // end of fn ss_json_href

/*
pub fn ss_json_href<'a>(page_json: &'a mut json::JsonValue, href: &str) -> Result<&'a mut json::JsonValue, ()> {
    // pub fn ss_json_href<'a>(page_json: &'a json::JsonValue, href: &str) -> Result<&'a mut json::JsonValue, ()> {

    let subsection_json = &mut page_json["data"]["subsection"]["data"];
    // subsection_json : { "0" : {}, "1" : {},}
    match subsection_json {
        json::JsonValue::Object(o) => {
            for ss_json_tup in o.iter_mut() {
                // ss_json_tup: ( 0: name, 1: value)
                println!("dom_to_json_utility fn ss_json_href ss_json_tup.0 : {}", &ss_json_tup.0);
                if ss_json_tup.0 == "href" {
                    let ss_json = ss_json_tup.1;
                    if ss_json["href"] == href {
                        return Ok(ss_json);
                    }
                }
            }
        },
        _ => (),
    }
    Err(())
} // end of fn ss_json_href
*/

// set sample ss_json if no any ss_json exists to let have a way to edit menu
pub fn subsection_json_sample_set(page_json: &mut json::JsonValue) {
    let top_id = 0;
    let ss_json_parent = ss_json_id(page_json, &top_id).unwrap();

    // already some ss_json
    if 0 < ss_json_parent["child"].len() {
        return;
    }

    // create new ss_json
    let href = "#sample";
    let ss_json = ss_json_new(page_json, &href, top_id);

    ss_json["href"] = href.into();
    ss_json["title"] = "sample".into();
}
