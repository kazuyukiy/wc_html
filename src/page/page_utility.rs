use regex;

// use crate::page5;
// use crate::page_json_utility;

// Create a new page under the parent_page
// It returns an instance of page4::Page
// but the file is not saved.
// You need to save the file if need it .
//
// If a file alraady exists in the child_href,
// use the previous file, so you will find the previous contents
// Create new navi data based on parent_page and add child_title to it
// converting href based on child_href
//
// parent_* : some value of parent_page
// child_* : some value of new navi data created in this function
//
// child_href: absolute or related location based on parent_page
//
// pub fn page_sub_new(parent_page: &mut page5::Page, child_title: &str, child_href: &str) -> Result<page5::Page, String> {
pub fn page_sub_new(
    // parent_page: &mut page5::Page,
    parent_page: &mut super::Page,
    child_title: &str,
    child_href: &str,
) -> Result<super::Page, String> {
    // println!("page5_utility.rs fn page_sub_new");

    let child_title = child_title.trim();
    if child_title.len() == 0 {
        return Err("no child title".to_string());
    }

    let child_href = child_href.trim();
    if child_href.starts_with("#") {
        return Err("child href starts with #".to_string());
    }
    if child_href.len() == 0 {
        return Err("child href.len is 0".to_string());
    }

    // let parent_url = parent_page.url.clone();
    let parent_url = &parent_page.url;

    let child_url = match parent_url.join(&child_href) {
        Ok(u) => u,
        Err(_) => return Err("parent_url.join failed".to_string()),
    };

    // let mut child_page = page5::Page::new(&child_url);
    let mut child_page = super::Page::new(&child_url);

    // let child_navi = match navi_parent_inherit_and_chld(&parent_page, &child_page) {
    // Ok(v) => v,
    // Err(e) => return Err(e),
    // };

    let page_json = match child_page.page_json.as_mut() {
        Some(v) => v.value_mut(),
        None => return Err("child_page.page_json.as_mut failed".to_string()),
    };

    // title
    page_json["data"]["page"]["title"] = child_title.into();

    // To set this title into child_navi,
    // page_json should have its value.
    let child_navi = match navi_parent_inherit_and_chld(&parent_page, &child_page) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let page_json = match child_page.page_json.as_mut() {
        Some(v) => v.value_mut(),
        None => return Err("child_page.page_json.as_mut failed".to_string()),
    };

    // navi
    page_json["data"]["navi"] = child_navi;

    // about child_page.page_json().value()["data"]["page"]["rev"]
    // Since the previous data will be used if it already exists,
    // rev will be inherited.
    // It will keep the previous backup files.
    // If rev would be reset to 1, it would overwite to the previous backup files .

    // Save will not be done automatically
    // Save it explicitly at another statement .
    // child_page.page_json_update_save();

    Ok(child_page)
} // end of fn page_sub_new

// Create navi data from parent_page converting href standing on parent_page
// fn navi_parent_inherit(parent_page: &page5::Page, child_page: &page5::Page) -> Result<json::JsonValue, String> {
fn navi_parent_inherit(
    parent_page: &super::Page,
    child_page: &super::Page,
) -> Result<json::JsonValue, String> {
    let parent_page_json = match parent_page.page_json.as_ref() {
        // Some(v) => v.value_mut(),
        Some(v) => v.value(),
        None => return Err("navi_parent_inherit page_json.as_ref() failed".to_string()),
    };

    // let parent_navi_jsv = &parent_page_json["data"]["navi"];
    // let parent_navi_jsv = match parent_navi_jsv {
    let parent_navi_jsv = match &parent_page_json["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err("navi_parent_inherit page_json.data.navi failed".to_string()),
    };

    let mut child_navi = json::JsonValue::new_array();

    for navi in parent_navi_jsv {
        let title = match navi[0].as_str() {
            Some(v) => v,
            None => "no title",
        };

        let href = match navi[1].as_str() {
            Some(parent_href) => {
                match href_base_switch(&parent_page.url, parent_href, &child_page.url) {
                    Some(v) => v,
                    None => "".to_string(),
                }
            }
            None => "".to_string(),
        };

        let vec: Vec<json::JsonValue> = vec![title.into(), href.into()];
        let _res = child_navi.push(json::JsonValue::Array(vec));
    }

    // temp
    Ok(child_navi)
} // end of fn navi_parent_inherit

// fn navi_parent_inherit_and_chld(parent_page: &page5::Page, child_page: &page5::Page) -> Result<json::JsonValue, String> {
fn navi_parent_inherit_and_chld(
    parent_page: &super::Page,
    child_page: &super::Page,
) -> Result<json::JsonValue, String> {
    // println!("page5_utility.rs fn navi_parent_inherit_and_chld");

    let mut child_navi = match navi_parent_inherit(&parent_page, &child_page) {
        Ok(v) => v,
        // Err(_) => return Err(()),
        // Err(_) => return Err("navi_parent_inherit failed".to_string()),
        Err(e) => return Err(e),
    };

    // set navi item of child_page
    let child_page_json = match &child_page.page_json {
        Some(v) => v.value(),
        // None => return Err(()),
        None => return Err("child_page.page_json not found".to_string()),
    };

    let title = match child_page_json["data"]["page"]["title"].as_str() {
        Some(s) => s,
        None => "no title",
    };
    let vec: Vec<json::JsonValue> = vec![title.into(), "".into()];
    let _res = child_navi.push(json::JsonValue::Array(vec));

    // println!("page5_utility.rs fn navi_parent_inherit_and_chld child_navi[last]: {:?}", &child_navi[child_navi.len()-1..child_navi.len()-1]);
    // println!("page5_utility.rs fn navi_parent_inherit_and_chld child_navi[last]: {:?}", &child_navi[child_navi.len()-1]);

    Ok(child_navi)
} // end of fn navi_parent_inherit_and_chld

// parent : where dest_page is related under it.
// parent value should be relative from dest_page
// If dest_page is not related under some page, leave parent as ""
//
// dest : where fm_page move to
//
// fm : what to move to dest
//
// fm_top : top of fm that will be moved at first,
// If fm have child pages, thoes pages will be moved recursively as well,
// and the top page of those pages is fm_top .
// If href in the pages is related to under fm_top,
// the relation shold be kept as relative
// because those pages move togather .
// But if href is related to where not under fm_top,
// the original relation shold be kept,
// but change the relation based on the new page location .
//
// pub fn page_move2(parent_url: Option<&url::Url>, dest_url: &url::Url, fm_page: &mut page5::Page) -> Result<(), String> {
pub fn page_move2(
    parent_url: Option<&url::Url>,
    dest_url: &url::Url,
    fm_page: &mut super::Page,
) -> Result<(), String> {
    println!("page5_utility.rs fn page_move2");

    // this check make possible to use fm_page.page_json.unwrap() later
    if let None = fm_page.page_json {
        return Err("page_move2 page_json is none".to_string());
    }

    // let mut dest_page = page5::Page::new(&dest_url);
    let mut dest_page = super::Page::new(&dest_url);
    // this check make possible to use dest_page.page_json.unwrap() later
    if let None = dest_page.page_json {
        return Err("page_move2 page_json is none".to_string());
    }

    // let mut dest_page = page5::Page::new(&dest_url);

    // Save dest_page if it already exists and no backup file saved .
    dest_page.page_current_save();

    let parent_page = match parent_url.as_ref() {
        // Some(url) => Some(page5::Page::from(&url)),
        Some(url) => Some(super::Page::from(&url)),
        None => None,
    };

    println!("page5_utility.rs fn page_move2 cp navi");

    // navi
    match page_move2_navi_set(parent_page.as_ref(), fm_page, &mut dest_page) {
        Ok(_) => (),
        // Err(e) => return Err(e),
        Err(e) => {
            println!("page5_utility.rs fn page_move2 Err:{}", &e);
            return Err(e);
        }
    };

    println!("page5_utility.rs fn page_move2 cp subsection");

    // subsections
    if let Err(e) = page_move2_subsection(fm_page, &mut dest_page) {
        println!(
            "page5_utility.rs fn page_move2 page_move2_subsection Err: {}",
            &e
        );

        return Err(e);
    };

    println!("page5_utility.rs fn page_move2 cp href_reference");

    if let Err(e) = page_move2_href_reference(fm_page, &mut dest_page) {
        return Err(e);
    };

    println!("page5_utility.rs fn page_move2 cp page_json_update_save");

    if let Err(_) = dest_page.page_json_update_save() {
        return Err("page_json_update_save was failed".to_string());
    };

    let _r = page_move2_fm_clear(fm_page, &dest_page);

    page_move2_children(&fm_page, &dest_page);

    Ok(())
} // end of fn page_move2

// fn page_move2_navi_set(parent_page: Option<&page5::Page>, fm_page: &page5::Page, dest_page: &mut page5::Page) -> Result<(), String> {
fn page_move2_navi_set(
    parent_page: Option<&super::Page>,
    fm_page: &super::Page,
    dest_page: &mut super::Page,
) -> Result<(), String> {
    // println!("page5_utility.rs fn page_move2_navi_set");

    // inherit parent_page's navi

    // fm_page.page_json is confirmes as it is Some at fn page_move2 .
    let fm_page_json = fm_page.page_json.as_ref().unwrap().value();

    let fm_page_navi = match &fm_page_json["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err("fm_page navi not found".to_string()),
    };

    // if only one navi element, it does not inherit parent navi
    // when parent_page2 has None, it will not inherit
    let parent_page2 = match parent_page.as_ref() {
        Some(v) => {
            // Only one navi item does not inherit parent navi
            if fm_page_navi.len() == 1 {
                // println!("page5_utility.rs fn page_move2_navi_set fm_page_navi.len == 1");

                None
            } else {
                // println!("page5_utility.rs fn page_move2_navi_set fm_page_navi.len != 1");

                Some(v)
            }
        }
        None => None,
    };

    let dest_navi = match parent_page2 {
        Some(parent_page) => match navi_parent_inherit_and_chld(parent_page, fm_page) {
            Ok(v) => v,
            Err(e) => return Err(e),
        },
        None => {
            let title = match fm_page_json["data"]["page"]["title"].as_str() {
                Some(s) => s,
                None => return Err("no fm_page title fond".to_string()),
            };

            let navi_vec: Vec<json::JsonValue> = vec![title.into(), "".into()];

            // println!("page5_utility.rs fn page_move2_navi_set navi_vec: {:?}", &navi_vec);

            let navi_ele = json::JsonValue::Array(navi_vec);
            // let navi: json::JsonValue = json::JsonValue::Array(navi_ele);
            // navi

            let mut navi = json::JsonValue::new_array();
            // let _ res = navi.push(navi_ele);
            match navi.push(navi_ele) {
                Ok(_) => (),
                Err(_) => return Err("navi.push failed".to_string()),
            }

            navi
        }
    };

    // dest_page.page_json is confirmed as it is Some at fn page_move2
    let dest_page_json = dest_page.page_json.as_mut().unwrap().value_mut();
    dest_page_json["data"]["navi"] = dest_navi;

    // println!("page5_utility.rs fn page_move2_navi_set dest_navi: {:?}", &dest_page_json["data"]["navi"]);

    Ok(())
} // end of fn page_move2_navi_set

// fn page_move2_subsection(fm_page: &page5::Page, dest_page: &mut page5::Page) -> Result<(), String> {
fn page_move2_subsection(fm_page: &super::Page, dest_page: &mut super::Page) -> Result<(), String> {
    // dest_page.page_json is confirmed as it is Some at fn page_mve2
    let dest_page_json = dest_page.page_json.as_ref().unwrap().value();

    // let dest_subsections = match &mut dest_page_json["data"]["subsection"]["data"] {
    let dest_subsections = match &dest_page_json["data"]["subsection"]["data"] {
        json::JsonValue::Object(v) => v,
        _ => return Err("dest_page subsection data not found".to_string()),
    };

    // If previous file exists on dest_page.url and
    // it has subsections expect top of subsections (id=0)
    // return Err and abort to create new page
    // To pass this, remove all subsections of the page
    // dest_subsections[0] : top of subsections
    if 1 < dest_subsections.len() {
        // return Err("dest_page already have subsection".to_string());
        return Err(format!(
            "dest_page {} already have subsection",
            &dest_page.url
        ));
    }

    let mut dest_subsections = dest_subsections.clone();

    // fm_page.page_json is confirmes as it is Some at fn page_move2 .
    let fm_page_json = fm_page.page_json.as_ref().unwrap().value();

    let fm_subsections = &fm_page_json["data"]["subsection"]["data"];
    for (id, fm_subsection) in fm_subsections.entries() {
        let mut dest_subsection = fm_subsection.clone();

        // href
        dest_subsection["href"] =
            match page_move2_subsection_href(fm_page, &fm_subsection, dest_page) {
                Ok(v) => v.into(),
                Err(e) => return Err(e),
            };

        // content
        dest_subsection["content"] =
            match page_move2_subsection_contents(fm_page, &fm_subsection, dest_page).into() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        _ = dest_subsections.insert(id, dest_subsection);
    }

    // dest_page.page_json is confirmed as it is Some at fn page_mve2
    let dest_page_json = dest_page.page_json.as_mut().unwrap().value_mut();
    dest_page_json["data"]["subsection"]["data"] = json::JsonValue::Object(dest_subsections);

    Ok(())
} // end of fn page_move2_subsection

// Convert href based on fm_page to based on dest_page
// If it is relation to fm_page or its children,
// relative href based on fm_page.url is as same as href based on dest_page.url
// location: fm_page/abc.html to dest_page/abc.html
// relation: ./abc.html becomes to be ./abc.html (no difference)
//
// Otherwise the absolut relation shoud be kept
// so href should be conveted based on fm_page.url to dest_page.url
// location ../dir2/abc.html
// relation: dest_dir/../some_dir/dir2/abc.html
// return relative url based on dest_page
// fn page_move2_href_convert(fm_page: &page5::Page, href: &str, dest_page: &page5::Page) -> Result<String, String> {
fn page_move2_href_convert(
    fm_page: &super::Page,
    href: &str,
    dest_page: &super::Page,
) -> Result<String, String> {
    let fm_href_url = match fm_page.url.join(&href) {
        Ok(v) => v,
        Err(_) => return Err("page_move2_href_convert fm_page.url.join(href) failed".to_string()),
    };

    let href_relative = match fm_page.url.make_relative(&fm_href_url) {
        Some(v) => v,
        None => {
            return Err(format!(
                "page_move2_href_convert href:{} failed to make_relative",
                href
            ))
        }
    };

    // if href_relative starts with ..
    // it relates not to fm_page or its children
    // In that case leave the relation but jus change its base to dest_page.url
    if href_relative.starts_with("../") {
        return match href_base_switch(&fm_page.url, href, &dest_page.url) {
            Some(v) => Ok(v),
            None => Err("page_move2_href_convert failed to href_base_switch".to_string()),
        };
    }

    // href is relation to some of fm_page or its children
    // relative based on fm_page.url can be used on dest_page.url
    match fm_page.url.make_relative(&fm_href_url) {
        Some(v) => Ok(v),
        None => Err("page_move2_href_convert dest_page.url.make_relative failed".to_string()),
    }
} // end of fn page_move2_href_convert

// convert href based on fm_page to dest_page
// refer fn page_move2_href_convert
// fn page_move2_subsection_href(fm_page: &page5::Page, fm_subsection: &json::JsonValue, dest_page: &page5::Page) -> Result<String, String> {
fn page_move2_subsection_href(
    fm_page: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &super::Page,
) -> Result<String, String> {
    let href = match fm_subsection["href"].as_str() {
        Some(v) => v,
        // None => return page_href_temp(&fm_page).ok_or("href of the subsection not found, temp href nether".to_string()),
        None => {
            let title = match fm_subsection["title"].as_str() {
                Some(v) => v,
                None => return Err("href is not found, nether its title".to_string()),
            };
            return Err(format!("title: {}: href is not found", &title));
        }
    };

    page_move2_href_convert(&fm_page, href, &dest_page)
} // end of fn page_move2_subsection_href

// fn page_move2_subsection_contents(fm_page: &page5::Page, fm_subsection: &json::JsonValue, dest_page: &page5::Page) -> Result<json::JsonValue, String> {
fn page_move2_subsection_contents(
    fm_page: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &super::Page,
) -> Result<json::JsonValue, String> {
    let fm_contents = match &fm_subsection["content"] {
        json::JsonValue::Array(v) => v,
        _ => return Err("fm_subsection content not found".to_string()),
    };

    let mut dest_contents = json::JsonValue::new_array();

    for fm_content in fm_contents {
        let fm_value = match fm_content["value"].as_str() {
            Some(v) => v,
            None => return Err("content.value not found".to_string()),
        };

        let dest_value =
            match page_move2_subsection_content_href_convert(fm_page, fm_value, dest_page) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        let mut dest_content = json::JsonValue::new_object();
        dest_content["type"] = fm_content["type"].clone();
        dest_content["value"] = dest_value.into();

        _ = dest_contents.push(dest_content);
    }

    Ok(dest_contents)
} // end of fn page_move2_subsection_contents

// fn page_move2_subsection_content_href_convert(fm_page: &page5::Page, fm_content: &str, dest_page: &page5::Page) -> Result<String, String> {
fn page_move2_subsection_content_href_convert(
    fm_page: &super::Page,
    fm_content: &str,
    dest_page: &super::Page,
) -> Result<String, String> {
    let mut content = String::from(fm_content);

    // inside of content["value"]
    let mut index = 0;
    loop {
        if index == content.len() {
            break;
        }
        if index > content.len() {
            break;
        } // timid

        // Find index of href value start and end in content_str
        // href="value1"
        // Search starts  from index position of content_str
        // let (start, end) = match href_pos(&content_str, index) {
        let (start, end) = match href_pos(&content, index) {
            Some(v) => v,
            None => break,
        };

        // next loop start from where after the current href value position
        index += end;

        // let href_value = &content_str[start..end];
        let href_value = &content[start..end];

        // Case #abc, it is local link, so nothing to chanve, leave as it is
        if href_value.starts_with("#") {
            continue;
        }

        let href_value = match page_move2_href_convert(&fm_page, href_value, &dest_page) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // replace with href_value
        content = content[0..start - 1].to_string() + &href_value + &content[end..];
    }

    Ok(content)
} // end of fn page_move2_subsection_content_href_convert

// fn page_move2_href_reference(fm_page: &page5::Page, dest_page: &mut page5::Page) -> Result<(), String> {
fn page_move2_href_reference(
    fm_page: &super::Page,
    dest_page: &mut super::Page,
) -> Result<(), String> {
    // fm_page.page_json is confirmes as it is Some at fn page_move2 .
    let fm_page_json = fm_page.page_json.as_ref().unwrap().value();

    let fm_href_reference = match &fm_page_json["data"]["href_reference"] {
        json::JsonValue::Object(v) => v,
        // If no data.href_reference, no need to the value to apply to dest_page
        // But it is not err matter
        _ => return Ok(()),
    };

    // dest_page.page_json is confirmed as it is Some at fn page_mve2
    // let dest_page_json = dest_page.page_json.as_mut().unwrap().value_mut();

    // let dest_href_reference = match &mut dest_page_json["data"]["href_reference"] {
    // json::JsonValue::Object(v) => v,
    // _ => return Err("dest_page href_reference data not found".to_string()),
    // };

    let mut dest_href_reference = json::JsonValue::new_object();

    // let dest_href_reference = match &mut dest_page_json["data"]["href_reference"] {
    // json::JsonValue::Object(v) => v,
    // _ => return Err("dest_page href_reference data not found".to_string()),
    // };

    for (href, dest) in fm_href_reference.iter() {
        let dest_str = match dest.as_str() {
            Some(v) => v,
            None => return Err("href_reference ".to_string()),
        };

        let dest: json::JsonValue = match page_move2_href_convert(&fm_page, dest_str, &dest_page) {
            Ok(v) => v.into(),
            Err(e) => return Err(e),
        };

        dest_href_reference.insert(href, dest);
    }

    // dest_page.page_json is confirmed as it is Some at fn page_mve2
    let dest_page_json = dest_page.page_json.as_mut().unwrap().value_mut();

    match dest_page_json["data"]["href_reference"] {
        json::JsonValue::Object(_) => (),
        _ => return Err("dest_page href_reference data not found".to_string()),
    };

    dest_page_json["data"]["href_reference"] = dest_href_reference;

    Ok(())
} // end of fn page_move2_href_reference

// fn page_move2_children(fm_page: &page5::Page, dest_page: &page5::Page) -> Result<(), String> {
fn page_move2_children(fm_page: &super::Page, dest_page: &super::Page) -> Result<(), String> {
    // println!("page5_utility.rs fn page_move2_children");

    // fm_page.page_json is confirmes as it is Some at fn page_move2 .
    let fm_page_json = fm_page.page_json.as_ref().unwrap().value();

    // let fm_page_navi = match &fm_page_json["data"]["navi"] {
    // json::JsonValue::Array(ref v) => v,
    // _ => return Err("fm_page navi not found".to_string()),
    // };

    let fm_subsections = &fm_page_json["data"]["subsection"]["data"];
    for (_id, fm_subsection) in fm_subsections.entries() {
        // println!("page5_utility.rs fn page_move2_children subsection id: {}", id);

        // href in fm_page
        let fm_child_href = match fm_subsection["href"].as_str() {
            Some(v) => v,
            None => continue,
        };

        if fm_child_href.len() == 0 {
            continue;
        }
        // href to local page
        if fm_child_href.starts_with("#") {
            continue;
        }

        // println!("page5_utility.rs fn page_move2_children fm_child_href: {}", fm_child_href);

        // url of href
        let fm_child_url = match fm_page.url.join(&fm_child_href) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // href relative basedn on fm_page
        let fm_child_relative = match fm_page.url.make_relative(&fm_child_url) {
            Some(v) => v,
            None => continue,
        };

        // Not fm_page's children
        if fm_child_relative.starts_with("../") {
            continue;
        };

        // println!("page5_utility.rs fn page_move2_children dest_page.url.path(): {}", &dest_page.url.path());

        // href based on dest_page
        // let dest_child_href = match href_base_switch(&fm_page.url, fm_child_href, &dest_page.url) {
        // Some(v) => v,
        // None => continue,
        // };

        // fn page_move2_href_convert(fm_page: &page5::Page, href: &str, dest_page: &page5::Page) -> Result<String, String> {

        let dest_child_href = match page_move2_href_convert(&fm_page, fm_child_href, &dest_page) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // println!("page5_utility.rs fn page_move2_children dest_child_href(): {}", &dest_child_href);

        // url of href after move to its destination
        let dest_child_url = match dest_page.url.join(&dest_child_href) {
            Ok(v) => v,
            // Err(_) => return Err("fn page_move2_children dest_page.url.join failed".to_string()),
            Err(_) => continue,
        };

        // println!("page5_utility.rs fn page_move2_children dest_child_url.path(): {}", &dest_child_url.path());

        // page that child of fm_page to move to destination
        // let mut fm_child_page = page5::Page::from(&fm_child_url);
        let mut fm_child_page = super::Page::from(&fm_child_url);

        page_move2(Some(&dest_page.url), &dest_child_url, &mut fm_child_page);
    }

    Ok(())
} // end of fn page_move2_children

// Clear fm_page data that was move to another location.
// fn page_move2_fm_clear(fm_page: &mut page5::Page, dest_page: &page5::Page) -> Result<(), String> {
fn page_move2_fm_clear(fm_page: &mut super::Page, dest_page: &super::Page) -> Result<(), String> {
    page_move2_fm_navi_set(fm_page, &dest_page);
    // page_move2_fm_contents_clear(fm_page, &dest_page);

    if let Err(_) = fm_page.page_json_update_save() {
        return Err("fm_page page_json_update_save was failed".to_string());
    };

    Ok(())
} // end of fn page_move2_fm_clear

// Remove all subsections contents
// fn page_move2_fm_navi_set(fm_page: &mut page5::Page, dest_page: &page5::Page) -> Result<(), String> {
fn page_move2_fm_navi_set(
    fm_page: &mut super::Page,
    dest_page: &super::Page,
) -> Result<(), String> {
    // println!("page5_urility fn page_move2_fm_navi_set");

    // fm_page.page_json is confirmes as it is Some at fn page_move2 .
    let fm_page_json = fm_page.page_json.as_mut().unwrap().value_mut();

    let fm_page_navi = match &mut fm_page_json["data"]["navi"] {
        json::JsonValue::Array(ref mut v) => v,
        _ => return Err("fm_page navi not found".to_string()),
    };

    let navi_len = fm_page_navi.len();
    // if fm_page_navi.len() == 0 { return Ok(()); }
    if navi_len == 0 {
        return Ok(());
    }

    let i_last = navi_len - 1;

    // let navi_last = match fm_page_navi[fm_page_navi.len()-1] {
    let navi_last = match fm_page_navi[i_last] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err("fm_page last navi not found".to_string()),
    };

    if navi_last.len() == 0 {
        return Err("fm_page last have has not value".to_string());
    }

    let title = match navi_last[0].as_str() {
        Some(v) => v,
        None => return Err("fm_page last has no value as str".to_string()),
    };

    println!("page5_urility fn page_move2_fm_navi_set title: {}", title);

    let title2 = format!("{} (moved to: {})", title, dest_page.url.path());

    fm_page_navi[i_last] = json::array![title2.as_str(), ""];

    Ok(())
} // end of fn page_move2_fm_navi_set

// // fn page_move2_fm_contents_clear(fm_page: &mut page5::Page, dest_page: &page5::Page) -> Result<(), String> {
// fn page_move2_fm_contents_clear(
//     fm_page: &mut super::Page,
//     dest_page: &super::Page,
// ) -> Result<(), String> {
//     // fm_page.page_json is confirmes as it is Some at fn page_move2 .
//     // let fm_page_json = fm_page.page_json.as_mut().unwrap().value_mut();

//     // let fm_page_navi = match &mut fm_page_json["data"]["navi"] {
//     // json::JsonValue::Array(ref v) => v,
//     // _ => return Err("fm_page navi not found".to_string()),
//     // };

//     Ok(())
// } // end of fn page_move2_fm_contents_clear

// Convert fm_page to dest_page under parent_page
//
// If a file already at dest_page.url ,
// inherit
// page_json["data"]["page"]["rev"] and
// page_json["data"]["href_reference"]
//
// use next rev number so the backup files with the preivious rev numbers
// would not be used and files would not be ober written .
//
// let keep previous href_refernece to keep the relationship .
// But if the previous page exits and same href key exists in the page,
// overwite the key that of page moving from to the previous pate's key .
//
// If somethig errors happen, stop moving so you can fix on the page you are moving to .
//
// pub fn page_move(fm_page: &mut page5::Page, mut dest_page: page5::Page, parent_page: Option<page5::Page>) -> Result<(), String> {
pub fn page_move(
    // fm_page: &mut page5::Page,
    // mut dest_page: page5::Page,
    // fm_page: &mut page5::Page,
    // mut dest_page: page5::Page,
    // parent_page: Option<page5::Page>,
    fm_page: &mut super::Page,
    mut dest_page: super::Page,
    parent_page: Option<super::Page>,
) -> Result<(), String> {
    // println!("page5_utility.rs fn page_move");

    // Save dest_page if it already exists and no backup file saved .
    dest_page.page_current_save();

    // This test is to let use &fm_page.page_json.unwrap() later
    if let None = &fm_page.page_json {
        return Err("fm_page.page_json not found".to_string());
    }

    // This test is to let use &dest_page.page_json.unwrap() later
    if let None = &dest_page.page_json {
        return Err("dest_page.page_json not found".to_string());
    }

    // let navi = match page_move_navi_set(&parent_page, fm_page) {
    match page_move_navi_set(&parent_page, fm_page, &mut dest_page) {
        Ok(_) => (),
        // Err(_) => return Err("failed to page_move_navi_set".to_string()),
        Err(e) => return Err(e),
    };

    if let Err(e) = page_move_subsection(fm_page, &mut dest_page) {
        return Err(e);
    };

    if let Err(e) = page_move_href_reference(fm_page, &mut dest_page) {
        return Err(e);
    };

    if let Err(_) = dest_page.page_json_update_save() {
        return Err("page_json_update_save was failed".to_string());
    };

    // temp
    Ok(())
} // end of fn page_move

// Case parent_page is given,
// Apply parent_page's navi and add a navi item of fm_page
//
// Case parent_page is not given,
// fm_page becomes a top navi element .
//
// fn page_move_navi_set(parent_page: &Option<page5::Page>, fm_page: &page5::Page, dest_page: &mut page5::Page) -> Result<json::JsonValue, String> {
fn page_move_navi_set(
    // parent_page: &Option<page5::Page>,
    // fm_page: &page5::Page,
    // parent_page: &Option<page5::Page>,
    // fm_page: &page5::Page,
    // dest_page: &mut page5::Page,
    parent_page: &Option<super::Page>,
    fm_page: &super::Page,
    dest_page: &mut super::Page,
) -> Result<(), String> {
    let res = match parent_page {
        Some(parent_page) => navi_parent_inherit_and_chld(&parent_page, fm_page),
        None => {
            // &fm_age.page_json was tested at the fn page_move, so can unwrap()
            let fm_page_json = &fm_page.page_json.as_ref().unwrap().value();
            let title = match fm_page_json["data"]["page"]["title"].as_str() {
                Some(s) => s,
                // None => "no title",
                None => return Err("no page title fond".to_string()),
            };
            let navi_ele: Vec<json::JsonValue> = vec![title.into(), "".into()];

            let navi: json::JsonValue = json::JsonValue::Array(navi_ele);
            Ok(navi)
        }
    };

    let navi = match res {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // dest_page.page_json was checkd at the beginning of fn page_save, so can unwrap()
    let dest_page_json = &mut dest_page.page_json.as_mut().unwrap().value_mut();

    dest_page_json["data"]["navi"] = navi;

    Ok(())
} // end of fn page_move_navi_set

// Create data subsection from fm_page based on dest_page
// fn page_move_subsection(fm_page: &page5::Page, dest_page: &mut page5::Page) -> Result<(), String> {
fn page_move_subsection(fm_page: &super::Page, dest_page: &mut super::Page) -> Result<(), String> {
    // let dest_page_json = match &mut dest_page.page_json {
    // Some(v) => v.value_mut(),
    // None => return Err("dest_page.page_json not found".to_string()),
    // };

    // dest_page.page_json was checkd at the beginning of fn page_save, so can unwrap()
    let dest_page_json = &mut dest_page.page_json.as_mut().unwrap().value_mut();

    let dest_subsections = match &mut dest_page_json["data"]["subsection"]["data"] {
        json::JsonValue::Object(v) => v,
        _ => return Err("dest_page subsection data not found".to_string()),
    };

    // If previous file exists on dest_page.url and
    // it has subsections , return Err and abort to create new page
    // To pass this, remove all subsections of the page
    if 1 < dest_subsections.len() {
        return Err("dest_page already have subsection".to_string());
    }

    // &fm_age.page_json was tested at the fn page_move, so can unwrap()
    let fm_page_json = &fm_page.page_json.as_ref().unwrap().value();

    let fm_subsections = &fm_page_json["data"]["subsection"]["data"];
    for (id, fm_subsection) in fm_subsections.entries() {
        let mut dest_subsection = fm_subsection.clone();

        // href
        dest_subsection["href"] =
            match page_move_subsection_href(fm_page, &fm_subsection, dest_page) {
                Ok(v) => v.into(),
                Err(e) => return Err(e),
            };

        // content
        dest_subsection["content"] =
            match page_move_subsection_contents(fm_page, &fm_subsection, dest_page).into() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        // dest_page.page_json was checkd at the beginning of fn page_save, so can unwrap()
        let dest_page_json = &mut dest_page.page_json.as_mut().unwrap().value_mut();
        let dest_subsections = &mut dest_page_json["data"]["subsection"]["data"];
        _ = dest_subsections.insert(id, dest_subsection);
    }

    Ok(())
} // end of fn page_move_subsection

fn page_move_subsection_href(
    // fm_page: &page5::Page,
    // fm_subsection: &json::JsonValue,
    // dest_page: &page5::Page,
    fm_page: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &super::Page,
) -> Result<String, String> {
    // Get href value
    // If no href data, return temporal value
    let href = match fm_subsection["href"].as_str() {
        Some(v) => v,
        None => {
            return page_href_temp(&fm_page)
                .ok_or("href of the subsection not found, temp href nether".to_string())
        }
    };

    match href_base_switch(&fm_page.url, href, &dest_page.url) {
        Some(v) => Ok(v),
        // if something wrong, return Err
        None => Err("failed to join href to new page".to_string()),
    }
} // end of fn page_move_subsection_href

// Create temporal local href
// that is not in used
// fn page_href_temp(page: &page5::Page) -> Option<String> {
fn page_href_temp(page: &super::Page) -> Option<String> {
    //fn page_href_not_used(page: &page5::Page) -> Option<String> {

    let page_json = match &page.page_json {
        Some(v) => v.value(),
        None => return None,
    };

    let subsections = match &page_json["data"]["subsection"]["data"] {
        json::JsonValue::Object(v) => v,
        _ => return None,
    };

    let href_pref = "#undefined".to_string();
    let mut i: u32 = 0;

    loop {
        let mut matched = false;
        // Scan all subsection items
        for (_id, subsection) in subsections.iter() {
            match subsection["href"].as_str() {
                Some(v) => {
                    // href_pref + i already exists
                    if &(href_pref.clone() + &i.to_string()) == v {
                        matched = true;
                        break;
                    }
                }
                None => (),
            }
        }

        // the href was not found
        if matched == false {
            return Some(href_pref + &i.to_string());
        }

        // the href was found, try href_pref + (i+1)
        i = match i.checked_add(1) {
            Some(v) => v,
            None => return None,
        };
    }

    // None
} // end of fn page_href_temp
  // } // end of fn page_href_not_used

fn page_move_subsection_contents(
    // fm_page: &page5::Page,
    // fm_subsection: &json::JsonValue,
    // dest_page: &page5::Page,
    fm_page: &super::Page,
    fm_subsection: &json::JsonValue,
    dest_page: &super::Page,
) -> Result<json::JsonValue, String> {
    let fm_contents = match &fm_subsection["content"] {
        json::JsonValue::Array(v) => v,
        _ => return Err("fm_subsection content not found".to_string()),
    };

    let mut dest_contents = json::JsonValue::new_array();

    for fm_content in fm_contents {
        let fm_value = match fm_content["value"].as_str() {
            Some(v) => v,
            None => return Err("content.value not found".to_string()),
        };

        let dest_value = page_move_subsection_content_href_convert(fm_page, fm_value, dest_page);

        let mut dest_content = json::JsonValue::new_object();
        dest_content["type"] = fm_content["type"].clone();
        dest_content["value"] = dest_value.into();

        _ = dest_contents.push(dest_content);
    }

    // temp
    Ok(dest_contents)
} // end of fn page_move_subsection_contents

fn page_move_subsection_content_href_convert(
    // fm_page: &page5::Page,
    // fm_content: &str,
    // dest_page: &page5::Page,
    fm_page: &super::Page,
    fm_content: &str,
    dest_page: &super::Page,
) -> String {
    let mut content = String::from(fm_content);

    // inside of content["value"]
    let mut index = 0;
    loop {
        // if index == content_str.len() { break; }
        // if index > content_str.len() { break; }
        if index == content.len() {
            break;
        }
        if index > content.len() {
            break;
        }

        // Find index of href value start and end in content_str
        // href="value1"
        // Search starts  from index position of content_str
        // let (start, end) = match href_pos(&content_str, index) {
        let (start, end) = match href_pos(&content, index) {
            Some(v) => v,
            None => break,
        };

        // next loop start from where after the current href value position
        index += end;

        // let href_value = &content_str[start..end];
        let href_value = &content[start..end];

        // Case #abc, it is local link, so nothing to chanve, leave as it is
        if href_value.starts_with("#") {
            continue;
        }

        // Convert href to basedn on dest_page
        let href_value = match href_base_switch(&fm_page.url, href_value, &dest_page.url) {
            Some(s) => s,
            // if not use href not converted
            None => href_value.to_string(),
        };

        // get content_str that href is replaced with what based on dest_page
        // let content_str = content_str[0..start-1].to_string()
        // + &href_value
        // + &content_str[end..];
        content = content[0..start - 1].to_string() + &href_value + &content[end..];

        // temp
        break;
    }

    content
} // end of fn page_move_subsection_content_href_convert

// Find position of href value in <a> element .
// from &str[find_start..]; scan starts from &str[find_start]
fn href_pos(str: &str, find_start: usize) -> Option<(usize, usize)> {
    // println!("papge5_utility.rs fn href_pos");

    let mut index = 0;
    let re_href = regex::Regex::new(r#"(?i)\s*href\s*=\s*["']"#).unwrap();

    // Search <a, but not \<a: escaped
    let (_a_start, a_end) = match find_not_escaped(&str, find_start, "<a") {
        Some(v) => v,
        // None => break,
        None => return None,
    };

    index += a_end;

    // Search href=" or (=')
    // if not found, search next <a
    let href_mat = match re_href.find(&str[index..]) {
        Some(v) => v,
        None => return None,
    };

    // first quote position of href="value"
    let q1_start = index + href_mat.end() - 1;

    let quote = &str[q1_start..q1_start + 1];

    index += href_mat.end();

    // Search second " (or ') of href="abc"
    let (q2_start, _q2_end) = match find_not_escaped(&str, index, quote) {
        Some(v) => v,
        None => return None,
    };

    // second quote position of href="value"
    // let quote_end = q2_start;

    // println!("papge5_utility.rs fn href_pos href value Try!");
    // println!("papge5_utility.rs fn href_pos href value: {}", &str[q1_start + 1..q2_start]);

    Some((q1_start + 1, q2_start))
} // end of fn href_pos

fn find_not_escaped(str_hole: &str, find_start: usize, ptn: &str) -> Option<(usize, usize)> {
    // let mut debug_mode = false;
    // debug_mode = false;
    // debug_mode = true;

    let str = &str_hole[find_start..];

    let re_ptn = regex::Regex::new(&ptn).unwrap();
    let re_esc = regex::Regex::new(r"(\\+)$").unwrap();

    let mut index_rest = 0;
    loop {
        if index_rest == str.len() {
            break;
        }
        if str.len() < index_rest {
            break;
        } // just in case

        let str_crt = &str[index_rest..];

        // find ptn
        let mat = re_ptn.find(&str_crt);

        // ptn was not found
        if let None = mat {
            break;
        }

        let mat = mat.unwrap();

        let a_index = mat.start();

        // if debug_mode {
        // println!("page_utility.rs find_not_escaped   ptn: {}", &ptn);
        // }

        // \ptn
        // if \ exists just befor ptn
        // it might be an escape code for ptn
        // or just \ charactor
        // In case of it is \ charactor,
        // it should be escape \ before \ charactor
        // so \\ is a caractor \.
        // If make couple of \ (\\) and still remains one \
        // it means ptn is escaped, that meas ptn is not an html element
        //
        let cap_op = re_esc.captures(&str_crt[..a_index]);
        if let Some(cap) = cap_op {
            // if debug_mode {
            // println!("page_utility.rs find_not_escaped esc: {}", &cap[1]);
            // }

            // escape
            if &cap[1].len() % 2 == 1 {
                index_rest += mat.end();
                continue;
            }
        }

        // if debug_mode {
        // println!("page_utility.rs find_not_escaped   ..: {}", &str_crt);
        // println!("page_utility.rs find_not_escaped  ..s: {}", &str_crt[..mat.start()]);
        // println!("page_utility.rs find_not_escaped s..e: {}", &str_crt[mat.start()..mat.end()]);
        // println!("page_utility.rs find_not_escaped  e..: {}", &str_crt[mat.end()..]);
        // }

        return Some((
            find_start + index_rest + mat.start(),
            find_start + index_rest + mat.end(),
        ));
    }

    None
} // end of fn find_not_escaped

// Convert fm_href basedn on fm_url to to_href basedn on to_url
// return relative url
fn href_base_switch(fm_url: &url::Url, fm_href: &str, to_url: &url::Url) -> Option<String> {
    match fm_url.join(&fm_href) {
        Ok(href_url) => {
            match to_url.make_relative(&href_url) {
                Some(v) => Some(v),
                // absolute
                None => Some(href_url.path().to_string()),
            }
        }
        Err(_) => None,
    }
} // end of fn href_base_switch

fn page_move_href_reference(
    // fm_page: &page5::Page,
    // dest_page: &mut page5::Page,
    fm_page: &super::Page,
    dest_page: &mut super::Page,
) -> Result<(), String> {
    // &fm_age.page_json was tested at the fn page_move, so can unwrap()
    let fm_page_json = &fm_page.page_json.as_ref().unwrap().value();

    let fm_href_reference = match &fm_page_json["data"]["href_reference"] {
        json::JsonValue::Object(v) => v,
        // _ => return Err("fm_page href_reference data is not found".to_string()),
        // If no data.href_reference, no need to the value to apply to dest_page
        // But it is not err matter
        _ => return Ok(()),
    };

    // dest_page.page_json was checkd at the beginning of fn page_save, so can unwrap()
    let dest_page_json = &mut dest_page.page_json.as_mut().unwrap().value_mut();

    let dest_href_reference = match &mut dest_page_json["data"]["href_reference"] {
        json::JsonValue::Object(v) => v,
        _ => return Err("dest_page href_reference data not found".to_string()),
    };

    // let fm_subsections = &fm_page_json["data"]["subsection"]["data"];
    // for (id, fm_subsection) in fm_subsections.entries() {
    for (href, dest) in fm_href_reference.iter() {
        dest_href_reference.insert(href, dest.clone());
    }

    Ok(())
} // end of fn page_move_href_reference
