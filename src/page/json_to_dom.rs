use std::rc::Rc;

// use crate::page_dom;
use super::page_dom;
use super::page_dom_utility::{attr, node_element, node_text};
// use crate::page_dom_utility::{attr, node_element, node_text};

use markup5ever_rcdom::{Node, NodeData};

pub fn json_to_dom(page_json: &json::JsonValue) -> page_dom::PageDom {
    let page_dom = page_dom::PageDom::new("");

    head_set(&page_dom, &page_json);
    body_set(&page_dom, &page_json);

    /*
    // create body contents
    //
    // wc.js will delete all body contents created here
    // and make new body contents on
    // <body onload="bodyOnload()">
    // from page_json data
    //
    // but considering on case wc.js does not work,
    // it is better to have html body contents in html file
    // so the page can be seen anyway.
    // that is why body contents will be created here
    //
    // But one problem is that there are two programs to create body contents,
    // here and wc.js.
    // The both programs should make same result. It makes difficult to maintain
    // to keep the both save.
    //
    // So far, while wc.js is working, programs here is not necessary,
    // make comment out this program.
    // We may think to resolve this issue later.


    edit_menu_set(&page_dom);
    edit_table_set(&page_dom);

    navi_set(&page_json, &page_dom);
    ul_set(&page_json, &page_dom);

    subsection_set(&page_json, &page_dom);
     */

    page_dom
}

fn head_set(page_dom: &page_dom::PageDom, page_json: &json::JsonValue) {
    let head = page_dom.head().unwrap();
    let mut head_children = head.children.borrow_mut();

    // title
    head_children.push(title_node(&page_json));
    // title_node(&page_json);
    // title_set(&page_json, &page_dom);

    // <meta charset="UTF-8">
    let meta = node_element("meta", &vec![("charset", "UTF-8")]);
    head_children.push(meta);

    // <script src="./../wc.js"></script>
    let script = node_element("script", &vec![("src", "/wc.js")]);
    head_children.push(script);

    // indivisual source may have own css
    // so that the contents can be seen without any external css file
    // however better to apply both cases, without and with external css file
    // <style type="text/css">
    head_children.push(style_node());
    // style_inhead_set(page_dom);

    // <link rel="stylesheet" href="./../wc.css">
    let link = node_element("link", &vec![("rel", "stylesheet"), ("href", "/wc.css")]);
    head_children.push(link);
} // end of fn head_set

fn title_node(page_json: &json::JsonValue) -> Rc<Node> {
    let mut title_str = "";
    if let Some(s) = page_json["data"]["page"]["title"].as_str() {
        title_str = s;
    }

    let title_node = node_element("title", &vec![]);
    let title_text = node_text(title_str);
    title_node.children.borrow_mut().push(title_text);

    title_node
} // end of fn title_node

fn style_node() -> Rc<Node> {
    let css = "
      .invisible { display: none; }

      .subsection {
	  margin-bottom: 20px;
      }
      .subsectionTitle {
	  font-weight: bold;
	  font-size: 1.5em;
      }
      .subsectionContent {
	  margin-left: 30px;
      }
      .subsectionSub {
	  margin-left: 30px;
      }
      .inline0 {
	  display: inline;
      }
      .script {
	  border: solid 1px black;
	  _width: 400px;
	  background: #e6e6fa;
	  padding: 10px;
	  resize: both;
	  overflow: hidden;
	  margin-bottom: 3px;
      }
";

    // .scriptSample {}

    let css_text = node_text(&css);

    // <style type="text/css">
    let style = node_element("style", &vec![("type", "text/css")]);
    style.children.borrow_mut().push(css_text);

    style

    // let head = page_dom.head().unwrap();
    // head.children.borrow_mut().push(style);
} // end of fn style_node

fn body_set(page_dom: &page_dom::PageDom, page_json: &json::JsonValue) {
    let body = page_dom.body().unwrap();

    // <body onload="bodyOnload()">
    if let NodeData::Element { ref attrs, .. } = body.data {
        let attr = attr("onload", "bodyOnload()");
        attrs.borrow_mut().push(attr);
    }

    let mut body_children = body.children.borrow_mut();

    // <span id="top"></span> is set by javascript, not at here .
    // // <span id="top"></span>
    // // Do this first to set the firstest part of the body.
    // let node_span_top = node_element("span", &vec![("id", "top")]);
    // body_children.push(node_span_top);

    // <span id="page_json_str" style="display: none"></span>
    let node_span_json = page_dom::node_span_page_json();

    // {...}
    let node_js_text = node_text(&page_json.to_string()); // text node
                                                          // <span id="page_json_str"></span>

    // <span id="page_json_str">{...}</span>
    node_span_json.children.borrow_mut().push(node_js_text);
    body_children.push(node_span_json);
} // end of fn body_set
