use std::fs;
// use std::fs::File;
use std::fs::DirBuilder;
use std::path::Path;

use regex::Regex;

pub struct PageFile {
    filename: String,
    content_bytes: Option<Vec<u8>>,
    content_str: Option<String>,
    debug_mode: bool,
} // end of struct PageFile

impl PageFile {
    pub fn open(filename: &str, debug_mode: bool) -> PageFile {
        let content_bytes = match fs::read(filename) {
            Ok(b) => Some(b),
            Err(_) => None,
        };

        let content_str;
        match content_bytes {
            Some(ref b) => match String::from_utf8(b.to_vec()) {
                Ok(s) => content_str = Some(s),
                Err(_) => content_str = None,
            },
            None => content_str = None,
        }

        PageFile {
            filename: filename.to_string(),
            content_bytes,
            content_str,
            debug_mode,
        }
    } // end of fn open

    pub fn filename(&self) -> &str {
        &self.filename
    } // end of fn filename

    pub fn content_bytes(&self) -> Option<&Vec<u8>> {
        if let None = self.content_bytes {
            return None;
        }
        let content = self.content_bytes.as_ref().unwrap();
        Some(content)
    } // fn content_bytes

    // pub fn content_str(&self) -> Option<&String> {
    pub fn content_str(&self) -> Option<&str> {
        if let None = self.content_str {
            return None;
        }
        let content = self.content_str.as_ref().unwrap();
        Some(content)
    } // fn content_str

    // check if self.content_str() contains str
    pub fn contains(&mut self, str: &str) -> bool {
        let content_str;
        match self.content_str() {
            Some(v) => {
                content_str = v;
            }
            // not UTF-8
            None => {
                return false;
            }
        }

        content_str.contains(str)
    } // end of fn contains

    // Replace content_bytes with the argument.
    // Replace contetn_str if the content_bytes can conver to UTF8.
    pub fn content_bytes_replace(&mut self, v8: Vec<u8>) {
        self.content_bytes = Some(v8);

        // Apply the change to self.content_str .
        match String::from_utf8(self.content_bytes.as_ref().unwrap().to_vec()) {
            Ok(s) => {
                self.content_str.replace(s);
            }
            Err(_) => {
                _ = self.content_str.take();
            }
        }
    } // end of fn content_bytes_replace

    // Replace content_str and content_bytes with the arguments.
    pub fn content_str_replace(&mut self, str: &str) {
        self.content_str.replace(String::from(str));
        // self.content_bytes.replace(self.content_str.as_ref().unwrap().into_bytes());
        // self.content_bytes.replace(self.content_str.as_ref().unwrap().to_string().into_bytes());
        let b = self.content_str.as_ref().unwrap().to_string().into_bytes();
        self.content_bytes.replace(b);
    } // end of fn content_str_replace

    // pub fn content_str_save(&self, str: &str) -> std::io::Result<()> {
    pub fn content_str_save(&self) -> std::io::Result<()> {
        let str = &self.content_str;
        if let None = str {
            return Err(result_err_other("no content"));
        }
        let str = str.as_ref().unwrap();

        if self.debug_mode == false {
            // let res = fs::write(&self.filename, str);

            // create dire
            dir_build(&self.filename);

            let res = fs::write(&self.filename, str);
            println!(
                "page_file3 fn content_str_save : {:?}: {}",
                &res, &self.filename
            );
            return res;
        } else {
            println!(
                "page_file3 fn content_str_save : {} - Not saved(debug mode)",
                &self.filename
            );
            return Err(result_err_other("debug mode"));
        }
    } // end of fn content_str_save

    pub fn save_as(&self, filename: &str) -> std::io::Result<()> {
        // Nodata to save
        if let None = self.content_bytes {
            return Err(result_err_other("no content_bytes"));
        }

        if self.debug_mode == false {
            // create dire
            dir_build(filename);

            let res = fs::write(filename, self.content_bytes.as_ref().unwrap());
            println!("page_file3 fn save_as : {:?}: {}", &res, filename);
            return res;
        } else {
            println!(
                "page_file3 fn save_as : {} - Not saved(debug mode)",
                filename
            );
            return Err(result_err_other("debug mode"));
        }
    } // end of fn save_as
} // end of impl PageFile

fn dir_build(filename: &str) {
    // filename : abc/def/ghi.html
    let re = Regex::new(r"/[^/]+$").unwrap();
    let mat = re.find(&filename).unwrap();

    // abc/def
    // &filename[..mat.start()]

    println!("page_file3 fn dir_build filename: {}", &filename);
    println!(
        "page_file3 fn dir_build path[]: {}",
        &filename[..mat.start()]
    );

    // let path = Path::new(&filename[..mat.start()]);

    // the path already exists .
    if Path::new(&filename[..mat.start()]).is_dir() {
        return;
    }

    DirBuilder::new()
        .recursive(true)
        .create(&filename[..mat.start()])
        .unwrap();

    // builder.create(path).unwrap();
} // end of fn dir_build

fn result_err_other(str: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, str)
} // end of fn result_err_other
