use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;

use httparse;

use crate::page::page_json_utility;
// use page;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
} // end of struct HttpRequest

impl HttpRequest {
    // println!("http_request4.rs impl HttpRequest");

    pub fn from(stream: &mut TcpStream) -> Option<HttpRequest> {
        // stream
        // Vec<u8>
        // starg-line
        // HTTP headers
        // empty line
        // body

        let stream_data = read_stream(stream);

        // println!("http_request4.rs impl HttpRequest fn from stream_data: {:?}", stream_data);

        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        // let res = req.parse(&stream_data);
        let res = match req.parse(&stream_data) {
            Ok(v) => v,
            Err(_) => return None,
        };

        // println!("http_request4.rs impl HttpRequest fn from req: {:?}", req);

        let method = match req.method {
            Some(s) => s.to_string(),
            None => return None,
        };

        let path = match req.path {
            Some(s) => s.to_string(),
            None => return None,
        };

        let mut header = HashMap::new();
        for header2 in req.headers.iter() {
            let key = header2.name.to_string();
            let value = String::from_utf8_lossy(&header2.value).into_owned();
            header.insert(key, value);
        }

        // // let res = req.parse(&stream_data);
        // let res = match req.parse(&stream_data) {
        // Ok(v) => v,
        // Err(_) => return None,
        // };

        let body_offset = match res {
            httparse::Status::Complete(v) => v,
            httparse::Status::Partial => return None,
        };

        let body = stream_data[body_offset..].to_vec();

        Some(HttpRequest {
            method,
            path,
            header,
            body,
        })
    } // end of fn from

    pub fn url(&self) -> Option<url::Url> {
        // println!("http_request4.rs impl HttpRequest fn url");
        let host = match self.header("Host") {
            Some(ref s) => s.to_owned(),
            None => return None,
        };

        let url = format!("https://{}{}", &host, &self.path);
        match url::Url::parse(&url) {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    } // end of fn url

    pub fn header(&self, name: &str) -> Option<&String> {
        self.header.get(name)
    } // end of fn header

    pub fn body_string(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    } // end of body_string

    pub fn body_json_value(&self) -> Option<json::JsonValue> {
        let body = &self.body_string();
        let page_json_str = page_json_utility::page_json_str_decode(&body);
        page_json_utility::page_json_value_from_str(&page_json_str)
    } // end of fn body_json_value
} // end of impl HttpRequest

fn read_stream(stream: &mut TcpStream) -> Vec<u8> {
    // const MESSAGE_SIZE: usize = 5;
    const MESSAGE_SIZE: usize = 1024;

    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    let mut recieved: Vec<u8> = vec![];

    loop {
        match stream.read(&mut rx_bytes) {
            Ok(bytes_read) => {
                recieved.extend_from_slice(&rx_bytes[..bytes_read]);
                if bytes_read < MESSAGE_SIZE {
                    break;
                }
            }

            Err(_) => {
                break;
            }
        }
    }

    recieved
} // end of fn read_stream
