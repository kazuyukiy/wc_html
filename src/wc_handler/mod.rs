use std::net::TcpStream;

use crate::page;

mod href_inspec;
mod http_request;

// Initialization
// fn system_ini() is to be called at beginning of main.rs main()
// so it will be done once at the biginning
// not every Tcp connection
//
// Copy wc.js, wc.css to
pub fn system_ini() {

    // under construction
} // end of fn system_ini

pub fn response(stream: &mut TcpStream) -> Vec<u8> {
    // println!("wc_handler5.rs fn response");

    // test_url_join();

    let request = match http_request::HttpRequest::from(stream) {
        Some(v) => v,
        // None => return response_404(),
        None => {
            println!("wc_handler5.rs fn response request not found");
            return response_404();
        }
    };

    let url = match request.url() {
        Some(v) => v,
        None => {
            println!("wc_handler5.rs fn response request.url: invalid");
            return response_404();
        }
    };

    // println!("wc_handler5.rs fn response url.path(): {}", url.path());
    // println!("wc_handler5.rs fn response method: {}", &request.method);

    println!(
        "wc_handler5.rs fn response {} {}",
        &request.method,
        url.path()
    );

    let mut handler = Handler::new(request);

    handler.response()

    // temp
    //	response_404()
} // end of fn response

fn response_200(source: &Vec<u8>) -> Vec<u8> {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        source.len()
    );

    // let res = header.into_bytes();

    [header.into_bytes(), source.clone()].concat()
} // fn response_200

fn response_404() -> Vec<u8> {
    let source = b"Not Found.".to_vec();

    let header = format!(
        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
        source.len()
    );

    [header.into_bytes(), source].concat()
} // end of fn response_404

pub struct Handler {
    pub request: http_request::HttpRequest,
    page: page::Page,
} // end of struct Handler

impl Handler {
    pub fn new(request: http_request::HttpRequest) -> Handler {
        let page = page::Page::from(&request.url().unwrap());

        Handler { request, page }
    } // end of fn new

    // fn page(&mut self) -> &mut page5::Page {
    // &mut self.page
    // } // end of fn page

    pub fn response(&mut self) -> Vec<u8> {
        if &self.request.method == "POST" {
            let v = match self.post_handle() {
                Ok(v) => v,
                // None => r#"{"res":"post_handle failed"}"#.to_string().into_bytes(),
                // {{ to escape {
                Err(e) => format!(r#"{{"res":"{}"}}"#, e).into_bytes(),
            };
            return response_200(&v);
        }

        if &self.request.method == "GET" {
            if let Some(v) = self.get_handle() {
                return response_200(&v);
            }
        };

        /*
        let res;
        if &self.request.method == "POST" {
            res = self.post_handle();
        } else if &self.request.method == "GET" {
            res = self.get_handle();
        } else {
            // res = Some(response_404());
            res = None;
        }

        match res {
            Some(v) => response_200(&v),
            // Some(v) => v,
            None => response_404(),
        }
         */

        response_404()
    } // end of fn response

    fn post_handle(&mut self) -> Result<Vec<u8>, String> {
        if self.page.is_wc_page() == false {
            return Err("not wc_page".to_string());
        }

        // xxx.html.01
        // It mean backup file,
        // ignore any request for backup
        if self.page.name_end_num() {
            return Err("backup file".to_string());
        }

        let request = match self.request.header("wc-request") {
            Some(v) => v,
            None => return Err("wc-request not found".to_string()),
        };

        println!("wc_handler5.rs fn post_handle request: {}", &request);

        if request == "json_save" {
            return self.page_json_save();
        }
        if request == "page_new" {
            return self.page_new();
        }
        if request == "onload" {
            return self.onload();
        }
        if request == "href" {
            return self.href_req();
        }
        if request == "page_move" {
            return self.page_move();
        }
        // temp
        // None

        // Some(r#"{"res":"post_handle failed"}"#.to_string().into_bytes())
        Err("request does not match".to_string())
    } // end of fn post_handle

    // Save page date in page_json sent from client.
    fn page_json_save(&mut self) -> Result<Vec<u8>, String> {
        // println!("wc_handler5 impl Handler fn page_json_save");

        let page_json = match self.request.body_json_value() {
            Some(v) => v,
            None => return Err("body_json_value not found".to_string()),
        };

        if let Err(_) = self.page.page_json_replace(page_json) {
            return Err("page.page_json_replace failed".to_string());
        }

        if let Err(_) = self.page.page_json_update_save() {
            return Err("page.page_json_update_save failed".to_string());
        }

        // Some(r#"{"res":"post_handle page_json_save"}"#.to_string().into_bytes())
        Ok(r#"{"res":"post_handle page_json_save"}"#.to_string().into_bytes())
    } // end of fn page_json_save

    // Create a new page and save.
    fn page_new(&mut self) -> Result<Vec<u8>, String> {
        // let this_page = wc_handler.page();
        // let this_page = self.page();

        // req: new page information
        // req["title"]
        // req["href"]
        let req = match self.request.body_json_value() {
            Some(v) => v,
            None => return Err("request.body_json_value failed".to_string()),
        };

        let title = match req["title"].as_str() {
            Some(s) => s,
            None => return Err("req.title not found".to_string()),
        };

        let href = match req["href"].as_str() {
            Some(s) => s,
            None => return Err("req.href not found".to_string()),
        };

        // let page_new = self.page.page_sub_new(title, href);
        // let mut page_new = match page_new {
        // Ok(v)	=> v,
        // Err(_) => return Err("page_new failed".to_string()),
        // };
        let mut page_new = match self.page.page_sub_new(title, href) {
            Ok(v) => v,
            Err(_) => return Err("page_new failed".to_string()),
        };

        _ = page_new.page_json_update_save();

        Ok(r#"{"res":"post_handle page_new"}"#.to_string().into_bytes())
    } // end of fn page_new

    fn onload(&self) -> Result<Vec<u8>, String> {
        let jv = json::object! {
            foo: "Mee",
            hee: "Moo Onload",
        };

        Ok(jv.to_string().into_bytes())
    } // end of fn onload

    fn href_req(&mut self) -> Result<Vec<u8>, String> {
        // println!("wc_handler5 impl Handler fn href_req");

        let caller_url = match self.request.url() {
            Some(v) => v,
            // None => return response_404(),
            None => return Err("request.url failed".to_string()),
        };

        // println!(
        //     "wc_handler5 impl Handler fn href_req caller_url: {}",
        //     caller_url
        // );

        // caller_url.host()

        let req = match self.request.body_json_value() {
            Some(v) => v,
            None => return Err("request.body_json_value not found".to_string()),
        };

        let url_req = req["href"].as_str().unwrap();

        // println!("wc_handler5 impl Handler fn href_req url_req: {}", url_req);

        let url_req2 = match url::Url::parse(&url_req) {
            Ok(u) => Some(u),
            Err(_) => None,
        };

        //	let url_dest2 = match url::Url::parse(&) {}

        // let url_caller = match &self.request.url() {
        //     Some(v) => Some(v),
        //     None => None,
        // };

        // if let Some(caller_url) = &self.request.url() {
        //     println!(
        //         "wc_handler5 impl Handler fn href_req caller_url: {}",
        //         &caller_url
        //     );
        // }

        // println!(
        //     "wc_handler5 impl Handler fn href_req url_req2: {:?}",
        //     url_req2
        // );

        // if let Some(req2) = url_req2 {
        // println!(
        //     "wc_handler5 impl Handler fn href_req req2.host: {:?}",
        //     req2.host()
        // );
        // if let Some(req_host) = req2.host() {
        // println!(
        //     "wc_handler5 impl Handler fn href_req req_host: {}",
        //     req_host
        // );

        // href_inspec5 impl HrefInspec fn href_req_handle url_valid: Some(Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(8080), path: "/Computing/computing_iroiro.html", query: None, fragment: None })

        // println!(
        //     "wc_handler5 impl Handler fn href_req self_host: {}",
        //     &self.request.url()
        // );
        // }
        // }

        let mut href_inspec = match href_inspec::HrefInspec::from(&caller_url, &url_req) {
            Ok(v) => v,
            Err(_) => return Err("href_indpec5 failed".to_string()),
        };

        // return href_inspec.href_req_handle();
        match href_inspec.href_req_handle() {
            Some(v) => Ok(v),
            None => Err("href_inspec.href_req_handle failed".to_string()),
        }
    } // end of fn href_req

    // fn page_move(&mut self) -> Option<Vec<u8>> {
    fn page_move(&mut self) -> Result<Vec<u8>, String> {
        let req = match self.request.body_json_value() {
            Some(v) => v,
            None => return Err("request.body_json_value failed".to_string()),
        };

        // req["parent_url"]
        // req["dest_url"]

        match self.page.page_move2(&req) {
            Ok(_) => Ok(r#"{"res":"post_handle page_move"}"#.to_string().into_bytes()),
            Err(e) => Err(e.to_string()),
        }

        /*
        let parent_url = match req["parent_url"].as_str() {
            Some(url_str) => {
                match self.page.url.join(url_str) {
                    Ok(url) => url,
                    Err(_) => return Err("req.parent_url.join failed".to_string()),
                }
            },
            None => return Err("parent_url failed".to_string()),
        };

        let parent_page = page5::Page::from(&parent_url);

        let dest_url = match req["dest_url"].as_str() {
            Some(url_str) => {
                match self.page.url.join(url_str) {
                    Ok(url) => url,
                    Err(_) => return Err("req.dest_url.join failed".to_string()),
                }
            },
            None => return Err("req.dest_url failed".to_string()),
        };

        // let mut dest_page = page5::Page::from(&dest_url);
        let dest_page = page5::Page::new(&dest_url);

        // let to_url = match req["url"].as_str() {
            // Some(v) => v,
            // None => return Err("req.url failed".to_string()),
        // };

        match self.page.page_move(dest_page, parent_page) {
            Ok(_) => Ok(r#"{"res":"post_handle page_move"}"#.to_string().into_bytes()),
            // Err(e) => Some(e.into_bytes())
            Err(e) => Err(e.to_string())
        }
         */

        // temp
        // Some(r#"{"res":"post_handle page_move"}"#.to_string().into_bytes())
    } // end of fn page_move

    fn get_handle(&mut self) -> Option<Vec<u8>> {
        // println!("wc_handler5.rs impl Handler fn get_handle");

        match self.page.source() {
            Some(v) => Some(v.to_vec()),
            None => None,
        }
    } // end of fn get_handle
} // end of impl Handler

fn test_url_join() {
    // println!("wc_handler5.rs fn test_url_join");

    let url = url::Url::parse("http://127.0.0.1/base/abc.html").unwrap();
    println!("wc_handler5.rs fn test_url_join url: {}", url);
    // wc_handler5.rs fn test_url_join url: http://127.0.0.1/base/abc.html

    // #loc becomes #loc as relative in same path
    let url2 = url.join("#logal_ref").unwrap();
    println!("");
    println!("wc_handler5.rs fn test_url_join url2 = join(#logal_ref)");
    println!("wc_handler5.rs fn test_url_join url2: {}", url2);
    // wc_handler5.rs fn test_url_join url2: http://127.0.0.1/base/abc.html#logal_ref
    println!(
        "wc_handler5.rs fn test_url_join relative: {}",
        url.make_relative(&url2).unwrap()
    );
    // wc_handler5.rs fn test_url_join relative: #logal_ref

    // def.html becomes def.html as relative in same path
    let url3 = url.join("def.html").unwrap();
    println!("");
    println!("wc_handler5.rs fn test_url_join url3 = url.join(def.html)");
    println!("wc_handler5.rs fn test_url_join url3: {}", url3);
    // wc_handler5.rs fn test_url_join url3: http://127.0.0.1/base/def.html
    println!(
        "wc_handler5.rs fn test_url_join relative: {}",
        url.make_relative(&url3).unwrap()
    );
    // wc_handler5.rs fn test_url_join relative: def.html

    // get relative if can be, use
    let url4 = url.join("http://127.0.0.1/base/def.html").unwrap();
    println!("");
    println!("wc_handler5.rs fn test_url_join url4 = url.join(http://127.0.0.1/base/def.html)");
    println!("wc_handler5.rs fn test_url_join url4: {}", url4);
    // wc_handler5.rs fn test_url_join url4: http://127.0.0.1/base/def.html
    match url.make_relative(&url4) {
        Some(v) => println!("wc_handler5.rs fn test_url_join url4 relative: {}", v),
        // wc_handler5.rs fn test_url_join url4 relative: def.html
        None => println!("wc_handler5.rs fn test_url_join url4 relative not found"),
    }

    // "" becomes "" as relative in same path
    println!("");
    println!("wc_handler5.rs fn test_url_join url5 = url.join(\"\")");
    match url.join("") {
        Ok(r) => {
            let url5 = r;
            println!("wc_handler5.rs fn test_url_join url5 join \"\" sucsess");
            // wc_handler5.rs fn test_url_join url5 join "" sucsess
            println!("wc_handler5.rs fn test_url_join url5: {}", url5);
            // wc_handler5.rs fn test_url_join url5: http://127.0.0.1/base/abc.html
            match url.make_relative(&url5) {
                Some(v) => println!("wc_handler5.rs fn test_url_join url5 relative: {}", v),
                // wc_handler5.rs fn test_url_join url5 relative:
                None => println!("wc_handler5.rs fn test_url_join url5 relative not found"),
            }
        }
        Err(e) => println!(
            "wc_handler5.rs fn test_url_join url5 join \"\" error: {:?}",
            e
        ),
    };

    // Case relative can not be
    let url6 = url.join("http://127.0.0.1/other/def.html").unwrap();
    println!("");
    println!("wc_handler5.rs fn test_url_join url6 = url.join(http://127.0.0.1/other/def.html)");
    println!("wc_handler5.rs fn test_url_join url6: {}", url6);
    // wc_handler5.rs fn test_url_join url6: http://127.0.0.1/other/def.html
    match url.make_relative(&url6) {
        Some(v) => println!("wc_handler5.rs fn test_url_join url6 relative: {}", v),
        // wc_handler5.rs fn test_url_join url6 relative: ../other/def.html
        None => println!("wc_handler5.rs fn test_url_join url6 relative not found"),
    }
} // end of fn test_url_join
