// use std::convert::AsMut;
// use std::convert::AsRef;

// use crate::page_file;
mod dom_to_json;
mod dom_to_json_utility;
mod json_to_dom;
mod page_dom;
mod page_dom_utility;
mod page_file;
mod page_json;
mod page_json_utility;
mod page_system_update;
mod page_system_update_0_0_1_to_0_0_3;
mod page_system_update_old_to_0_0_3;
mod page_utility;

use regex::Regex;

pub struct Page {
    pub url: url::Url,
    file: page_file::PageFile,
    // page_json: Option<page_json3::PageJson>,
    pub page_json: Option<page_json::PageJson>,
} // end of struct Page

impl Page {
    // println!("page5.rs");

    #[warn(unused_assignments)]
    fn debug_mode() -> bool {
        // true // debut mode
        false // not debug mode
    } // end of fn debug_mode

    // Create an instance of Page
    // If Page.file.content_str is None or page_json data not found in there
    // set empty page_json data
    //
    pub fn new(url: &url::Url) -> Page {
        let mut page = Page::from(url);
        if let None = page.page_json {
            // page.page_json.as_mut().unwrap().value_replace(page_json_utility::page_json_blank());

            let page_json = page_json::PageJson::new(page_json_utility::page_json_blank());
            page.page_json.replace(page_json);
        }

        page
    } // end of fn new

    // Create an instance of Page
    // If page_json data is not found in Page.file.content_str(),
    // leace Page.page_json as None
    pub fn from(url: &url::Url) -> Page {
        let filename = page_filename(&url.path());
        let file = page_file::PageFile::open(&filename, Page::debug_mode());

        // Find json data in str from the content
        // and convert it to json value.
        let page_json = match file.content_str() {
            Some(s) => match page_json_utility::page_json_value_from_content(s) {
                Some(v) => Some(page_json::PageJson::new(v)),
                None => None,
            },
            None => None,
        };

        Page {
            url: url.clone(),
            file,
            page_json,
        }
    } // end of fn from

    pub fn source(&mut self) -> Option<&Vec<u8>> {
        // DBG comment out
        self.page_system_update();
        println!("page5.rs DBG comment outed page_system_update");

        self.file.content_bytes()
    } // end of fn source

    fn page_system_update(&mut self) {
        page_system_update::update(self);
    } // end of fn page_system_update

    // xxx.html.01
    // It mean backup file
    pub fn name_end_num(&self) -> bool {
        // xxx.html.01
        let re = Regex::new(r"\d+$").unwrap();
        re.is_match(&self.file().filename())
    } // end of fn name_end_num

    pub fn file(&self) -> &page_file::PageFile {
        &self.file
    } // end of fn file

    pub fn page_json_replace(&mut self, page_json: json::JsonValue) -> Result<(), ()> {
        if let None = self.page_json {
            return Err(());
        }

        self.page_json.as_mut().unwrap().value_replace(page_json);

        Ok(())
    } // end of fn page_json_replace

    fn filename_rev_next(&mut self) -> Option<String> {
        let filename = self.file().filename().to_string();

        let page_json = match self.page_json.as_mut() {
            Some(v) => v,
            // Some(ref mut v) => v,
            None => return None,
        };

        let filename_rev_next = filename + "." + &page_json.rev_next().to_string();

        Some(filename_rev_next)
    } // end of fn filename_rev_next

    // Save page as contents that read from the file .
    // No change is applied on the save
    // event you made any change on page.page_jsson
    // To save chages of page_json, use
    //  fn page_json_update_save()
    //
    // Save the curent data with the current rev.
    // fn page_current_save may be failed because filename_rev may already exists.
    // Because fn page_json_update_save might make filename_rev_next.
    // But this software is being changed always and
    // not sure is current page was saved.
    pub fn page_current_save(&mut self) {
        let rev = match self.page_json.as_mut() {
            Some(page_json) => page_json.rev().to_string(),
            None => return,
        };

        let filename_rev = self.file().filename().to_string() + "."
			// + &self.page_json().rev().to_string();
			+ &rev;

        // If filname_rev exists.
        if let Ok(_) = std::fs::File::open(&filename_rev) {
            return;
        }

        _ = self.file().save_as(&filename_rev);
    } // end of fn page_current_save

    // Save page applying changes on selr.page_json .
    //
    // Create a new page data from page_json, and save it.
    pub fn page_json_update_save(&mut self) -> Result<(), ()> {
        // Get and set rev to next value
        // before convert page_json to dom and get its html
        // to apply next rev value to new page
        // page_json["data"]["page"]["rev"]
        let filename_rev_next = match self.filename_rev_next() {
            Some(v) => v,
            None => return Err(()),
        };

        // Make a new contents in string from page_json
        // let page_json = self.page_json().value();
        // let page_json = &page_json.value();
        let page_json = match self.page_json.as_mut() {
            Some(v) => v.value_mut(),
            None => return Err(()),
        };

        // page_json to dom
        let page_dom = json_to_dom::json_to_dom(&page_json);

        // dom to html string
        let html = page_dom.to_html();

        // replace new content
        self.file.content_str_replace(&html);

        // Save the new content to the file.
        _ = self.file.content_str_save();
        // Save a backup.
        _ = self.file.save_as(&filename_rev_next);

        Ok(())
    } // end of fn page_json_update_save

    // pub fn page_sub_new(&mut self, new_title: &str, new_href: &str) -> Result<Page, String> {
    pub fn page_sub_new(&mut self, new_title: &str, new_href: &str) -> Result<Page, String> {
        page_utility::page_sub_new(self, new_title, new_href)
    } // end of fn page_sub_new

    // parent : where dest_top put under it .
    // dest : where fm move to
    // fm : what to move to dest
    // fm_top : top of fm that will be moved at first,
    // If fm have child pages, thoes pages will be moved recursively as well,
    // and the top of those pages is fm_top .
    // If href in the pages is related to under fm_top,
    // the relation shold be kept as relative
    // because those pages move togather .
    // But if href is related to where not under fm_top,
    // the original relation shold be kept,
    // but change the relation based on the new page location .
    pub fn page_move2(&mut self, req: &json::JsonValue) -> Result<Vec<u8>, String> {
        // req["parent_url"]
        // req["dest_url"]

        let parent_url = match req["parent_url"].as_str() {
            Some(v) => v.trim(),
            None => return Err("page5.page_move2 req.parent_url failed".to_string()),
        };

        let parent_url = if parent_url.len() == 0 {
            None
        } else {
            match self.url.join(parent_url) {
                Ok(url) => Some(url),
                Err(_) => return Err("page5.page_move2 parent_url join failed".to_string()),
            }
        };

        let dest_url = match req["dest_url"].as_str() {
            Some(url_str) => match self.url.join(url_str) {
                Ok(url) => url,
                Err(_) => return Err("page5.page_move2 dest_url join failed".to_string()),
            },
            None => return Err("page5.page_move2 req.dest_url failed".to_string()),
        };

        let _res = match page_utility::page_move2(parent_url.as_ref(), &dest_url, self) {
            Ok(_) => (),
            // Err(e) => Err(e),
            Err(_) => (),
        };

        // temp
        Ok(r#"{"res":"page5.rs page_move2"}"#.to_string().into_bytes())
    } // end of fn page_move2

    // pub fn page_move(&self, to_url: &str) -> Option<Page> {
    // pub fn page_move(&mut self, dest_page: Page, parent_page: Page) -> Option<Page> {
    pub fn page_move(&mut self, dest_page: Page, parent_page: Page) -> Result<(), String> {
        match page_utility::page_move(self, dest_page, Some(parent_page)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } // end of page_move

    // Check whether it is a wc page.
    pub fn is_wc_page(&mut self) -> bool {
        // If file contents is not utf8,
        //  self.file.contains("xx") always returns false
        // not html
        if self.file.contains("<body") == false && self.file.contains("<BODY") == false {
            return false;
        }

        // no wc keyword founded measn is not wc data
        // atleast one of those should be containd
        if {
            self.file.contains("naviBase")
                || self.file.contains("wc_top")
                || self.file.contains("WC_top")
                || self.file.contains("subsection")
        } == false
        {
            return false;
        }

        true
    } // end of fn is_wc_page
} // end of impl Page

// impl AsMut for Page {
// fn as_mut
// } // end of impl AsMut for Page

/*
impl AsRef<Page> for Page {
    fn as_ref(&self) -> &Page {
        &self
    } // end of fn as_ref
} // end of impl AsRef for Page
*/

fn page_filename(uri: &str) -> String {
    "pages".to_owned() + &uri
} // end of fn page_filename
