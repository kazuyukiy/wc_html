use std::rc::Rc;

use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
// use regex::Regex;
use tendril;

// use crate::page_dom_utility::*;
//use crate::page_dom_utility as pdu;
use super::page_dom_utility as pdu;

pub struct PageDom {
    pub dom: RcDom,
}

impl PageDom {
    pub fn new(source: &str) -> PageDom {
        let parser = parse_document(RcDom::default(), Default::default());
        let dom = parser.one(source);

        PageDom { dom }
    } // end of fn new

    pub fn child_match_first(&self, ptn: &Node) -> Option<Handle> {
        let list = pdu::child_match_list(&self.dom.document, ptn, true);
        if list.len() < 1 {
            return None;
        }

        let child = Rc::clone(&list[0]);
        Some(child)
    } // end of fn child_match_first

    pub fn head(&self) -> Option<Handle> {
        // <head>

        self.child_match_first(&pdu::node_element("head", &vec![]))
    } // end of fn head

    pub fn body(&self) -> Option<Handle> {
        self.child_match_first(&pdu::node_element("body", &vec![]))
    } // end of fn body

    // pub fn page_json_str(&self) -> Option<String> {
    //     if let Some(t) = self.span_page_json_str() {
    //         return Some(String::from(&t));
    //     }

    //     if let Some(s) = self.script_page_json_str() {
    //         return Some(s);
    //     }

    //     None
    // } // end of fn page_json_str

    // span element to enclose page_json in text string
    pub fn span_page_json(&self) -> Option<Handle> {
        let span_ptn = node_span_page_json();
        self.child_match_first(&span_ptn)
    }

    // page_json in text string
    // pub fn page_json_str(&self) -> Option<&str> {
    pub fn span_page_json_str(&self) -> Option<tendril::StrTendril> {
        let node = self.span_page_json();
        if let None = node {
            return None;
        }
        let node = node.unwrap();
        let children = node.children.borrow();
        if children.len() == 0 {
            return None;
        }
        match &children[0].data {
            NodeData::Text { contents } => {
                // return Some(contents);
                //             ^^^^^^^^
                // `&RefCell<Tendril<UTF8>>`
                return Some(contents.borrow().clone());
            }
            _ => return None,
        }
    } // end of fn span_page_json_str

    pub fn script_page_json(&self) -> Option<Handle> {
        // <script type="text/javascript" class="page_json">let page_json = {}</script>
        let script_ptn = node_script_page_json();

        // create new script node if not exists
        if self.child_match_first(&script_ptn).is_none() {
            match self.head() {
                // if <head> exists
                Some(head) => {
                    // create <script> for page_json
                    let script = node_script_page_json();
                    head.children.borrow_mut().push(script);
                }
                None => return None,
            }
        }

        // get node script
        let script;
        match self.child_match_first(&script_ptn) {
            Some(n) => script = n,
            None => return None,
        }

        Some(script)
    } // end of fn script_page_json

    // pub fn script_page_json_str(&self) -> Option<String> {
    //     // <script type="text/javascript" class="page_json">let page_json = {...}</script>

    //     // script element has no chld
    //     if self.script_page_json().unwrap().children.borrow().len() < 1 {
    //         return None;
    //     }

    //     match &self.script_page_json().unwrap().children.borrow()[0].data {
    //         NodeData::Text { contents } => {
    //             // err contents.trim();

    //             // let page_json = {...}
    //             let reg_ptn = r"^\s*let\s+page_json\s*=\s*(\{.*\})\s*$";
    //             let contents_borrow = contents.borrow();
    //             let op_pj = match_one(reg_ptn, &contents_borrow);
    //             match op_pj {
    //                 // Some(pj) => Some(pj.to_string()),
    //                 Some(pj) => {
    //                     // Some(pj.to_string())
    //                     // pj.to_string() : &lt;td&gt;TAB&lt;/td&gt;

    //                     let pj_str = pdu::entity_reference_unset(pj);

    //                     Some(pj_str)
    //                 }
    //                 None => None,
    //             }
    //         }
    //         _ => None,
    //     }
    // } // end of fn script_page_json_str

    pub fn serialize(&self, node: &Handle) -> String {
        /*
        // DBG
        let sh = SerializableHandle::from(node.clone());
        let mut bytes = vec![];
        serialize(&mut bytes, &sh, SerializeOpts::default()).unwrap();
        let html = String::from_utf8(bytes).unwrap();
        println!("page_dom fn serialize html: {}", html);
         */

        let sh = SerializableHandle::from(node.clone());
        let mut bytes = vec![];
        serialize(&mut bytes, &sh, SerializeOpts::default()).unwrap();

        String::from_utf8(bytes).unwrap()
        // &lt;td&gt;TAB&lt;/td&gt;
        // "test":"\"&amp;apm;"
    } // end of fn serialize

    pub fn to_html(&self) -> String {
        let str = self.serialize(&self.dom.document);
        format!("<!DOCTYPE html>{}", str)
    } // end of fn to_html
}

// fn match_one<'a>(reg_ptn: &str, obj: &'a str) -> Option<&'a str> {
//     let reg = Regex::new(reg_ptn).unwrap();

//     let op_cap = reg.captures(&obj);
//     let cap;
//     match op_cap {
//         Some(m) => {
//             cap = m;
//         }
//         None => {
//             return None;
//         }
//     }

//     let match1;
//     let op_get = cap.get(1);
//     match op_get {
//         Some(m) => match1 = m,
//         None => {
//             // println!("Could not get one");
//             return None;
//         }
//     }

//     let str_match = &match1.as_str();

//     if str_match.len() == 0 {
//         return None;
//     }

//     Some(&str_match)
// }

pub fn node_script_page_json() -> Rc<Node> {
    // <script type="text/javascript" class="page_json">let page_json = {}</script>
    pdu::node_element(
        "script",
        &vec![("type", "text/javascript"), ("class", "page_json")],
    )
}

pub fn node_span_page_json() -> Rc<Node> {
    // <span id="page_json_str" style="display: none"></span>
    let att = vec![("id", "page_json_str"), ("style", "display: none")];
    pdu::node_element("span", &att)
}
