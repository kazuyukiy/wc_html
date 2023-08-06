use std::cell::RefCell;
use std::rc::Rc;

// use json;
use markup5ever::interface::Attribute;
use markup5ever::{namespace_url, ns};
use markup5ever::{LocalName, QualName};
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom};
use tendril::Tendril;
// use regex::Regex;
// use regex;

pub fn node_element(ele_name: &str, attrs_vec: &Vec<(&str, &str)>) -> Rc<Node> {
    Node::new(NodeData::Element {
        name: qual_name(ele_name),
        attrs: attrs(&attrs_vec),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
} // end of fn node_element

pub fn qual_name(name: &str) -> QualName {
    QualName::new(
        None,
        // ns!(html), // <script unknown_namespace:type="text/javascript">
        ns!(), // <script type="text/javascript">
        // Namespace::from("http://abc.rs"),
        LocalName::from(name),
        // local_name!(name),
    )
} // end of fn qual_name

fn attrs(attrs_vec: &Vec<(&str, &str)>) -> RefCell<Vec<Attribute>> {
    let mut attr_list: Vec<Attribute> = vec![];
    for (name, value) in attrs_vec {
        attr_list.push(attr(&name, &value));
    }
    RefCell::new(attr_list)
} // end of fn attrs

pub fn attr(name: &str, value: &str) -> Attribute {
    Attribute {
        name: qual_name(&name),
        value: Tendril::from(value.to_string()),
    }
} // end of fn attr

pub fn node_text(text: &str) -> Rc<Node> {
    Node::new(NodeData::Text {
        // contents: RefCell::new(Tendril::from(&text[..])),
        contents: RefCell::new(Tendril::from(text)),
    })
} // end of fn node_text

pub fn child_match_list(node_obj: &Handle, node_ptn: &Node, recursive: bool) -> Vec<Handle> {
    let mut node_list: Vec<Handle> = vec![];
    node_child_match(node_obj, node_ptn, &mut node_list, recursive);
    node_list
} // end of fn child_match_list

fn node_child_match(
    node_obj: &Handle,
    node_ptn: &Node,
    node_list: &mut Vec<Handle>,
    recursive: bool,
) {
    for child in node_obj.children.borrow().iter() {
        if element_match(child, node_ptn) {
            node_list.push(Rc::clone(child));
        }

        if recursive {
            node_child_match(child, node_ptn, node_list, recursive);
        }
    }
} // end of fn node_child_match

fn element_match(node: &Handle, node_ptn: &Node) -> bool {
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let name_org = name;
            let attrs_org = attrs;
            match node_ptn.data {
                NodeData::Element {
                    ref name,
                    ref attrs,
                    ..
                } => {
                    // both node and node_ptn are Element
                    if name_org.local == name.local {
                        // see if all attrs of node_ptn match attrs of node
                        return attrs_match(attrs_org.clone(), attrs.clone());
                    }
                    false
                }
                // node_ptn is not Element
                _ => false,
            }
        }
        // node is not Element
        _ => return false,
    }
} // end of fn element_match

fn attrs_match(attrs: RefCell<Vec<Attribute>>, attrs_ptn: RefCell<Vec<Attribute>>) -> bool {
    if 0 == attrs_ptn.borrow().len() {
        // no attrs condition
        return true;
    }

    for att_for in attrs_ptn.borrow().iter() {
        'attrs: loop {
            for att in attrs.borrow().iter() {
                if att_for.name.local == att.name.local {
                    if att_for.value == att.value {
                        // att_for match att
                        // see next att_for
                        break 'attrs;
                    } else {
                        // value does not match
                        return false;
                    }
                }
            }
            // att_for.name does not exists in att
            return false;
        } // end of 'attrs: loop
    }

    // all of attrs_ptn match attrs
    true
} // end of fn attrs_match

pub fn json_in_javascript(dom: &RcDom) -> Result<json::JsonValue, ()> {
    let doc = &dom.document;
    let node_json_ptn = node_element("div", &vec![("class", "page_json")]);

    let list = child_match_list(doc, &node_json_ptn, true);
    if list.len() < 1 {
        return Err(());
    }

    for child in list[0].children.borrow().iter() {
        match child.data {
            NodeData::Text { ref contents, .. } => {
                let js = contents.borrow().to_string();
                // remove 'page_json = '
                let js = js.replace("let page_json = ", "");
                let page_json_res = json::parse(&js);
                match page_json_res {
                    Ok(page_json) => {
                        return Ok(page_json);

                        // To Do
                        // Change return type ;
                    }
                    _ => return Err(()),
                }
            }
            _ => (),
        }
        return Err(());
    }

    Err(())
} // end of fn json_in_javascript

pub fn entity_reference_unset(str: &str) -> String {
    let ptn = r#"(&(?:(?:lt)|(?:gt));)"#;
    let re = regex::Regex::new(ptn).unwrap();
    let result = re.replace_all(str, |caps: &regex::Captures| {
        // entity_reference_to_str(&caps[1]) ).into_owned()
        match entity_reference_to_str(&caps[1]) {
            Some(s) => s.to_string(),
            None => caps[1].to_string(),
        }
    });

    result.to_string()
}

fn entity_reference_to_str<'a>(er: &'a str) -> Option<&'a str> {
    if er == "&lt;" {
        return Some("<");
    }
    if er == "&gt;" {
        return Some(">");
    }

    None
}
/*
*/

/*
fn entity_reference_to_str<'a, 'b>(str: &'a str) -> &'b str {
// fn entity_reference_to_str<'a>(str: &'a str) -> &'a str {

    // DBG
    println!("page_dom_urility fn entity_reference_to_str str: {}", str);


    if str == "&lt;" { return "<"}
    if str == "&gr;" { return ">"}


    let str2 = String::from(str);
    &str2

        // let str2: &str = str.clone();
    // str2
    // str.clone()

    // temporary
    // ""

}
*/
