// Convert html page to json value

use std::cell::RefCell;
use std::rc::Rc;

use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use markup5ever::interface;
use markup5ever_rcdom::{Handle, Node, NodeData, SerializableHandle}; // RcDom,
use tendril::StrTendril; // Tendril,

// use crate::page_dom_utility::*;
use super::dom_to_json_utility as dom_json_utility;
use super::page_dom_utility as dom_utility;
use super::page_json_utility as json_utility;

//use super::page_dom_json as dom_json;
// use crate::page_dom_utility;
// use crate::page_json_utility;

// use crate::dom_to_json_utility::*;

// This is to convert genaral html style page to json data in text base
// Analyze dom parsed from html source and convert to json text data.
pub fn dom_to_json(node: &Handle) -> json::JsonValue {
    // let mut page_json = page_json_utility::page_json_blank();
    let mut page_json = json_utility::page_json_blank();

    content_rev_set(node, &mut page_json);

    navi_json_set(node, &mut page_json);

    index_json_set(node, &mut page_json);

    subsection_json_set(node, &mut page_json);

    subsection_json_set_2(node, &mut page_json);

    // subsection_json_sample_set(&mut page_json);
    dom_json_utility::subsection_json_sample_set(&mut page_json);

    // DBG
    // page_json["test"] = r#""&apm;"#.into();
    // println!("dom_to_json fn dom_to_json page_json[test]: {}", page_json["test"]);
    // page_json["test"] : "

    page_json
}

fn content_rev_set(node: &Handle, page_json: &mut json::JsonValue) {
    // <script id="editModeRevSet">editModeRev = 0</script><pre id="editModeRevCount" class="invisible">6</pre>
    let content_rev_ptn = dom_utility::node_element("pre", &vec![("id", "editModeRevCount")]);
    let pre_list = dom_utility::child_match_list(node, &content_rev_ptn, true);
    if pre_list.len() < 1 {
        return;
    }

    let mut content_rev = 0;
    for child in pre_list[0].children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents, .. } => {
                let rev_str = contents.borrow().to_string();
                let i_result = u32::from_str_radix(&rev_str, 10);
                match i_result {
                    Ok(i) => {
                        content_rev = i;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    page_json["data"]["page"]["rev"] = content_rev.into();
} // end of fn content_rev_set

fn navi_json_set(node: &Handle, page_json: &mut json::JsonValue) {
    let a_list = navi_a_list(node);

    for node in &a_list {
        match node.data {
            // <a href="#abc">Name</a>
            NodeData::Element { ref attrs, .. } => {
                let mut href = String::new();
                let mut name = String::new();
                // href
                for attr in attrs.borrow().iter() {
                    if &attr.name.local == "href" {
                        href = String::from(&attr.value);
                    }
                }

                // name
                for child in node.children.borrow().iter() {
                    match child.data {
                        NodeData::Text { ref contents, .. } => {
                            // consider to use trim()
                            name = contents.borrow().to_string().replace("\n", "");
                        }
                        _ => (),
                    }
                    break;
                }

                // page_json["data"]["navi"][[name0, href0], ...]
                let navi_json = json::array![name, href];
                let _r = page_json["data"]["navi"].push(navi_json);
            }
            _ => (),
        }
    }

    let navi_list = &page_json["data"]["navi"];
    let navi_len = navi_list.len();
    if 0 < navi_len {
        let title = navi_list[navi_len - 1][0].as_str().into();
        page_json["data"]["page"]["title"] = title;
    }
} // end of fn navi_json_set

fn navi_a_list(node: &Handle) -> Vec<Handle> {
    let a_list = navi_a_list_navi_base(node);

    if 0 < a_list.len() {
        return a_list;
    }

    // let a_list = navi_a_list_top_html(node);
    let a_list = navi_list_from_body(node);

    a_list
} // end of fn navi_a_list

fn navi_a_list_navi_base(node: &Handle) -> Vec<Handle> {
    // "naviBase"
    let base_ptn = dom_utility::node_element("div", &vec![("class", "naviBase")]);
    let base_list = dom_utility::child_match_list(node, &base_ptn, true);
    if base_list.len() < 1 {
        return vec![];
    }

    // "a"
    let a_ptn = dom_utility::node_element("a", &vec![]);
    dom_utility::child_match_list(&base_list[0], &a_ptn, true)
} // end of fn navi_a_list_navi_base

// Find <a href="wc_top.html"> from body
fn navi_top_from_body(node: &Handle) -> Option<Handle> {
    // <body>
    // Some case <a> found in <head>, that is wrong way.
    // However to ensure to find <a> from <body>
    let body_ptn = dom_utility::node_element("body", &vec![]);
    let body_list = dom_utility::child_match_list(node, &body_ptn, true);
    if body_list.len() < 1 {
        return None;
    }

    // <a>
    let a_ptn = dom_utility::node_element("a", &vec![]);
    // let a_list_all = dom_utility::child_match_list(node, &a_ptn, true);
    let a_list_all = dom_utility::child_match_list(&body_list[0], &a_ptn, true);

    // <a href="wc_top.html">
    let mut a_top: Option<Handle> = None;
    for a in &a_list_all {
        if let NodeData::Element { ref attrs, .. } = a.data {
            for att in attrs.borrow_mut().iter() {
                if att.name.local == dom_utility::qual_name("href").local {
                    // wc_top.html or WC_top.html or top.html
                    if att.value.contains("top.html") {
                        _ = att.value.replace("WC_top.html", "wc_top.html");
                        // a_node_match = a_node;
                        a_top.replace(Rc::clone(&a));
                        // a_top = a;
                        break;
                    }
                }
            }
        }
    }

    a_top
}

// Return navi_list referring fn navi_top_from_body
fn navi_list_from_body(node: &Handle) -> Vec<Handle> {
    let a_top_op = navi_top_from_body(node);
    if a_top_op.is_none() {
        return vec![];
    }

    let parent_weak_op = a_top_op.unwrap().parent.take();
    if parent_weak_op.is_none() {
        return vec![];
    }

    let parent_weakhandle = parent_weak_op.unwrap();
    let parent_upgrade_op = parent_weakhandle.upgrade();
    if parent_upgrade_op.is_none() {
        return vec![];
    }
    let parent = parent_upgrade_op.unwrap();

    let mut navi_list: Vec<Handle> = vec![];
    let mut navi_last_text = None;
    'c: for child in parent.children.borrow().iter() {
        match child.data {
            NodeData::Element { ref attrs, .. } => {
                if attrs.borrow().len() == 0 {
                    continue;
                }
                for attr in attrs.borrow().iter() {
                    // <a href="#abc">ABC</a>
                    if &attr.name.local == "href" {
                        navi_list.push(Rc::clone(child));
                        // if Element exists, ignore test before the element.
                        _ = navi_last_text.take();
                        continue 'c;
                    }
                }
            }
            // Might be a text not included in <a> that is of the current page
            NodeData::Text { ref contents } => {
                // contents.borrow();
                // `Ref<'_, Tendril<UTF8>>`
                navi_last_text.replace(contents.borrow().to_string());
            }
            _ => {}
        }
    }

    // Case some text after <a></a> that is the current page name.
    // <a></a>&gt;&gt;Last Name
    // the letest words should be the current page name.
    if let Some(last_text) = navi_last_text {
        let mut last: Vec<&str> = last_text.trim().split("\n").collect();
        if let Some(last_name) = last.pop() {
            let a_last = dom_utility::node_element("a", &vec![]);
            let node_text = dom_utility::node_text(last_name);
            a_last.children.borrow_mut().push(node_text);
            navi_list.push(a_last);
        }
    }

    navi_list
}

fn index_json_set(node: &Handle, page_json: &mut json::JsonValue) {
    let ul_list = index_ul_list(node);
    if ul_list.len() < 1 {
        return;
    }

    // ul_list[0] is considered as the top ul
    // let pid: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    // index_ul_json_set(&ul_list[0], page_json, Rc::clone(&pid));
    let pid = 0;
    index_ul_json_set(&ul_list[0], page_json, pid);
} // end of fn index_json_set

fn index_ul_list(node: &Handle) -> Vec<Handle> {
    let ul_list = index_ul_list_base(node);

    if 0 < ul_list.len() {
        return ul_list;
    }

    index_ul_list_ul(node)
} // end of fn index_ul_list

fn index_ul_list_base(node: &Handle) -> Vec<Handle> {
    // li
    // <ul class="listItemBase">
    // <li class="listItem">
    // <a class="title" href="./web/web_index.html">Web</a>

    let ul_ptn = dom_utility::node_element("ul", &vec![("class", "listItemBase")]);
    dom_utility::child_match_list(node, &ul_ptn, true)
} // end of fn index_ul_list_base

fn index_ul_list_ul(node: &Handle) -> Vec<Handle> {
    let ul_ptn = dom_utility::node_element("ul", &vec![]);
    dom_utility::child_match_list(&node, &ul_ptn, true)
} // end of fn index_ul_list_ul

fn index_ul_json_set(node: &Handle, page_json: &mut json::JsonValue, pid: u32) {
    let li_ptn = dom_utility::node_element("li", &vec![]);
    let li_list = dom_utility::child_match_list(&node, &li_ptn, true);
    let a_ptn = dom_utility::node_element("a", &vec![]);
    for li in li_list {
        let mut href = String::new();
        let mut name = String::new();
        let a_list = dom_utility::child_match_list(&li, &a_ptn, false);
        for a in a_list.iter() {
            match a.data {
                NodeData::Element { ref attrs, .. } => {
                    // href
                    for attr in attrs.borrow().iter() {
                        if &attr.name.local == "href" {
                            href = String::from(&attr.value);
                            break;
                        }
                    }

                    // name
                    for child in a.children.borrow().iter() {
                        match child.data {
                            NodeData::Text { ref contents, .. } => {
                                name = contents.borrow().to_string().replace("\n", "");
                                break;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }

            let href_str = String::from(&href);
            // let ss_json = ss_json_new(page_json, &href_str, Rc::clone(&pid));
            let ss_json = dom_json_utility::ss_json_new(page_json, &href_str, pid);
            ss_json["title"] = name.into();
            // ss_json["href"] = href.into();
            ss_json["href"] = href_str.into();

            break;
        }

        // go to sub ul
        let ul_ptn = dom_utility::node_element("ul", &vec![("class", "listItemBase")]);
        let ul_list = dom_utility::child_match_list(&li, &ul_ptn, true);
        if 0 < ul_list.len() {
            let pid;
            let ss_json_res = dom_json_utility::ss_json_href(page_json, &href);
            match ss_json_res {
                // Ok(ss_json) => {
                Some(ss_json) => {
                    // let pid_v = json_value_to_u32(&mut ss_json["parent"]);
                    // pid = Rc::new(RefCell::new(pid_v));
                    match ss_json["parent"].as_u32() {
                        Some(i) => pid = i,
                        None => pid = 0,
                    }
                }
                // _ => pid = Rc::new(RefCell::new(0)),
                _ => pid = 0,
            }

            // index_ul_json_set(&ul_list[0], page_json, Rc::clone(&pid));
            index_ul_json_set(&ul_list[0], page_json, pid);
        }
    }
} // end of fn index_ul_json_set

fn subsection_json_set(node: &Handle, page_json: &mut json::JsonValue) {
    /*
       <div class="subsection" id="href_target">
         <div class="subsectionTitle">The Title</div>
         <div class="subsectionContent">
           <div class="textContent"></div>
           <div class="textContent scriptSample"></div>
           <div class="htmlContent"></div>
           <div class="htmlContent subsectionSub"></div>
         </div>
       </div>
    */

    //<div class="subsection" id="href_target">
    let subsection_ptn = dom_utility::node_element("div", &vec![("class", "subsection")]);
    let subsection_div_list = dom_utility::child_match_list(node, &subsection_ptn, true);
    // for each subsection
    'subsection: for subsection_div in subsection_div_list.iter() {
        match subsection_div.data {
            NodeData::Element { ref attrs, .. } => {
                for attr in attrs.borrow().iter() {
                    // attribute id has has href target as its value
                    if &attr.name.local == "id" {
                        if subsection_set(page_json, subsection_div, &attr.value) {
                            continue 'subsection;
                        }
                    }
                }
            }
            // not node element
            _ => (),
        }
    }
} // end of fn subsection_json_set

fn node_subsection_title(node_subsection: &Rc<Node>) -> Option<String> {
    let title_for = dom_utility::node_element("div", &vec![("class", "subsectionTitle")]);
    let title_list = dom_utility::child_match_list(&node_subsection, &title_for, false);
    // title not found
    if title_list.len() < 1 {
        return None;
    }

    let title_contents = title_list[0].children.borrow();
    if title_contents.len() < 1 {
        return None;
    }

    match title_contents[0].data {
        NodeData::Text { ref contents } => {
            return Some(contents.borrow().to_string());
        }
        _ => {
            return None;
        }
    }
} // end of fn node_subsection_title

// attrs: attribue list of the element
// find attribute name is "class"
// and its value contains v
// <div class="htmlContent"></div>
// class_contains(atrrs, "htmlContent")
fn class_contains(attrs: &RefCell<Vec<interface::Attribute>>, v: &str) -> bool {
    attrs.borrow().iter().any(|attr| {
        if &attr.name.local != "class" {
            return false;
        }
        attr.value.split_whitespace().any(|e| e == v)
    })
}

fn subsection_set(page_json: &mut json::JsonValue, subsection_div: &Handle, id: &str) -> bool {
    // subsectionContent
    // <div class="subsectionContent">
    let att_ptn = vec![("class", "subsectionContent")];
    let subsection_content_ptn = dom_utility::node_element("div", &att_ptn);
    let subsection_content_list =
        dom_utility::child_match_list(subsection_div, &subsection_content_ptn, true);
    if subsection_content_list.len() < 1 {
        return false;
    }

    // <div class="htmlContent subsectionSub"></div>
    // <div class="htmlContent"></div>
    // <div class="textContent scriptSample"></div>
    // <div class="textContent"></div>
    let content_div_ptn = dom_utility::node_element("div", &vec![]);
    let content_div_list =
        dom_utility::child_match_list(&subsection_content_list[0], &content_div_ptn, false);
    if content_div_list.len() < 1 {
        return false;
    }

    // if &attr.value.to_string() == "subsection_template" { return false;}
    if id == "subsection_template" {
        return false;
    }

    let href = format!("#{}", &id);

    ss_json_prepare(page_json, subsection_div, &href);

    // DBG
    // let ss_json = ss_json_href(page_json, &href).unwrap();
    // println!("dom_to_json fn subsection_set href: {}", &href);

    let ss_json = dom_json_utility::ss_json_href(page_json, &href).unwrap();

    // discon  not "children", but "child" set in fn ss_json_new
    // ss_json["children"] = json::array![];

    // content set
    // content_set(&content_div_list, &mut ss_json);
    content_set(&content_div_list, ss_json);

    true
} // end of fn subsection_set

// create a new ss_json if not found with href
fn ss_json_prepare(page_json: &mut json::JsonValue, subsection_div: &Handle, href: &str) {
    // ss_json with the href exists
    // if let Ok(_) = ss_json_href(page_json, &href) { return; }
    if let Some(_) = dom_json_utility::ss_json_href(page_json, &href) {
        return;
    }

    let title = node_subsection_title(subsection_div).unwrap_or("no title".to_string());
    let pid = 0;
    let ss_json = dom_json_utility::ss_json_new(page_json, href, pid);

    // discon href is set in fn ss_json_new
    // ss_json["href"] = href.into();

    ss_json["title"] = title.into();
}

//
// < class = "htmlContent">  -- "type" : "html",
// < class = "scriptSample"> -- "type" : "script",
// < class = "textContent">  -- "type" : "text",
fn content_set(content_div_list: &Vec<Handle>, ss_json: &mut json::JsonValue) {
    for content_div in content_div_list.iter() {
        match content_div.data {
            NodeData::Element { ref attrs, .. } => {
                // < class = "htmlContent">
                if class_contains(attrs, "htmlContent") {
                    let html = content_html_parse(content_div);
                    // <td>TAB</td>

                    // DBG
                    // println!("dom_to_json fn content_set html: {}", html);

                    let mut content_json = json::object! {
                        "type" : "html",
                    };
                    content_json["value"] = html.into();
                    // <td>TAB</td>

                    // DBG
                    // println!("dom_to_json fn content_set content_json[value]: {}", content_json["value"]);

                    let _r = ss_json["content"].push(content_json);
                    // continue 'content_div;
                    continue;
                }

                // < class = "scriptSample">
                // script_sample
                if class_contains(attrs, "scriptSample") {
                    // ss_json; `&mut JsonValue`
                    content_script_set(content_div, ss_json);
                    // continue 'content_div;
                    continue;
                }

                // < class = "textContent">
                // text
                if class_contains(attrs, "textContent") {
                    content_text_set(content_div, ss_json);
                }
            }
            _ => (),
        }
    }
} // end of fn content_set

fn content_html_parse(node: &Rc<Node>) -> String {
    let sh = SerializableHandle::from(node.clone());
    let mut bytes = vec![];
    serialize(&mut bytes, &sh, SerializeOpts::default()).unwrap();
    String::from_utf8(bytes).unwrap()
} // end of fn content_html_parse

// wc.js function divToVariable
// content_div: <div class="textContent scriptSample"></div>
//
// text
// br
// pre + class: inline0
// div recursive fn content_script_set
//
// { "tyle" : "script", "value" : "text data"}
fn content_script_set(content_div: &Rc<Node>, ss_json: &mut json::JsonValue) {
    let mut script = String::new();

    for child in content_div.children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents, .. } => {
                script.push_str(&contents.borrow());
            }
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                // <br>
                if &name.local == "br" {
                    script.push_str("\n");
                }
                // <pre class="inline0">text</pre>
                if &name.local == "pre" {
                    if attrs.borrow().iter().any(|attr| {
                        &attr.name.local == "class" && &attr.value.to_string() == "inline0"
                    }) {
                        match child.children.borrow()[0].data {
                            NodeData::Text { ref contents } => {
                                script.push_str(&contents.borrow());
                            }
                            _ => (),
                        }
                    }
                } // end of pre
            }
            _ => (),
        }
    }

    // { "tyle" : "script", "value" : "text data"}
    let mut content_json = json::object! {
        "type" : "script",
    };
    content_json["value"] = script.into();
    let _r = ss_json["content"].push(content_json);
} // end of fn content_script_set

// { "tyle" : "text", "value" : "text data"}
fn content_text_set(content_div: &Rc<Node>, ss_json: &mut json::JsonValue) {
    for child in content_div.children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents, .. } => {
                let mut content_json = json::object! {
                    "type" : "text",
                };
                content_json["value"] = contents.borrow().to_string().into();
                let _r = ss_json["content"].push(content_json);
            }
            _ => (),
        }
    }
} // end of fn content_text_set

fn subsection_json_set_2(node: &Handle, page_json: &mut json::JsonValue) {
    // for old style

    // <div class="subsection">
    let subsection_div_ptn = dom_utility::node_element("div", &vec![("class", "subsection")]);
    let subsection_div_list = dom_utility::child_match_list(node, &subsection_div_ptn, true);

    for subsection_div in subsection_div_list.iter() {
        subsection_set2(page_json, &subsection_div);
    }
} // end of fn subsection_json_set_2

fn subsection_set2(page_json: &mut json::JsonValue, subsection_div: &Handle) {
    // href
    // <a name="href_name"></a>
    let href = subsection_href_2(Rc::clone(&subsection_div));
    if href.len() == 0 {
        return;
    }

    // <div class="subsectionBody">
    let body_ptn = dom_utility::node_element("div", &vec![("class", "subsectionBody")]);
    let div_body_list = dom_utility::child_match_list(&subsection_div, &body_ptn, true);
    if div_body_list.len() == 0 {
        return;
    }

    ss_json_prepare2(page_json, subsection_div, &href);
    let ss_json = dom_json_utility::ss_json_href(page_json, &href).unwrap();

    content_set2(&div_body_list, ss_json);
}

fn subsection_href_2(subsection_div: Handle) -> String {
    // href
    // <a name="href_name"></a>
    // let mut href = String::new();
    let a_href_ptn = dom_utility::node_element("a", &vec![]);
    let a_list = dom_utility::child_match_list(&subsection_div, &a_href_ptn, true);
    for a_ele in a_list {
        match a_ele.data {
            NodeData::Element { ref attrs, .. } => {
                for attr in attrs.borrow().iter() {
                    if &attr.name.local == "name" {
                        // href = attr.value.to_string();
                        // href = format!("#{}", attr.value.to_string());
                        return format!("#{}", attr.value.to_string());
                        // break;
                    }
                }
            }
            _ => (),
        }
    }

    String::new()
}

fn ss_json_prepare2(page_json: &mut json::JsonValue, subsection_div: &Handle, href: &str) {
    // if let Ok(_) = ss_json_href(page_json, &href) { return; }
    if let Some(_) = dom_json_utility::ss_json_href(page_json, &href) {
        return;
    }

    // ss_json with the haref not exists
    let ss_json = dom_json_utility::ss_json_new(page_json, &href, 0);

    // title
    let mut title = "no title".to_string();
    // <div class="subsectionTitle">
    let title_ptn = dom_utility::node_element("div", &vec![("class", "subsectionTitle")]);
    let div_title_list = dom_utility::child_match_list(&subsection_div, &title_ptn, true);
    if 0 < div_title_list.len() {
        let title_contents = div_title_list[0].children.borrow();
        if 0 < title_contents.len() {
            match title_contents[0].data {
                NodeData::Text { ref contents } => {
                    // remove "\n"
                    let contents_borrow = contents.borrow();
                    let title_list: Vec<&str> = contents_borrow.split("\n").collect();
                    for title_part in title_list.iter() {
                        if 0 < title_part.len() {
                            title = title_part.to_string();
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    ss_json["title"] = title.into();
}

fn content_set2(div_body_list: &Vec<Handle>, ss_json: &mut json::JsonValue) {
    let children = div_body_list[0].children.borrow();

    let mut content_json_vec = json::array![];
    for child in children.iter() {
        match child.data {
            // html or script
            NodeData::Element { ref name, .. } => {
                // content_json_vec, name, child
                content_element_set(&mut content_json_vec, name, &child);
            }
            NodeData::Text { ref contents } => {
                content_text_set2(&mut content_json_vec, contents.clone());
            }
            _ => (),
        }
    }

    ss_json["content"] = content_json_vec;
}

fn content_element_set(
    content_json_vec: &mut json::JsonValue,
    name: &interface::QualName,
    child: &Handle,
) {
    if name.local.to_string() == "textarea" {
        // type : script
        // <textarea class="scriptSample">
        let mut text_content: Vec<String> = vec![];
        for textarea_child in child.children.borrow().iter() {
            match textarea_child.data {
                NodeData::Text { ref contents } => {
                    text_content.push(contents.borrow().to_string());
                }
                _ => (),
            }
        }

        let content_json = json::object! {
            "type" : "script",
            "value" : text_content.join("\n"),
        };
        let _r = content_json_vec.push(content_json);
    } else {
        // type: html
        // element but not textarea
        let html = content_html_parse(child);
        let content_json = json::object! {
            "type" : "html",
            "value" : html
        };
        let _r = content_json_vec.push(content_json);
    }
}

fn content_text_set2(content_json_vec: &mut json::JsonValue, contents: RefCell<StrTendril>) {
    let mut content_json = json::object! {
        "type" : "text",
    };
    let mut text_vec: Vec<&str> = vec![];
    let contents_str = contents.borrow().to_string();
    for content in contents_str.split("\n").collect::<Vec<&str>>().iter() {
        if 0 < content.len() {
            text_vec.push(content);
        }
    }
    content_json["value"] = text_vec.join("\n").into();
    let _r = content_json_vec.push(content_json);
}
