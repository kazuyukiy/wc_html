use std::fs::File;
use std::io::Write;

mod css_source;
mod js_source;

// Setup javascript and css file
pub fn setup() {
    for dtype in ["js", "css"] {
        handle_dtype(dtype);
    }
}

fn handle_dtype(dtype: &str) {
    // page/wc.js, page/wc.css
    let filename = "pages/wc.".to_string() + dtype;

    let source = match dtype {
        "js" => js_source::contents(),
        "css" => css_source::contents(),
        _ => "",
    };

    match std::fs::OpenOptions::new().write(true).open(&filename) {
        Ok(mut file) => match write(&mut file, &source) {
            Ok(_) => println!("wrote: {}", &filename),
            Err(_) => println!("failed to write: {}", &filename),
        },
        // not exists
        Err(_) => {
            new_write(&filename, &source);
        }
    }
}

fn new_write(filename: &str, source: &str) {
    match File::create(&filename) {
        // new file
        Ok(mut file) => match write(&mut file, source) {
            Ok(_) => println!("write: {}", filename),
            Err(_) => println!("failed to write : {}", filename),
        },
        Err(e) => {
            println!("failed to create file: {}", filename);
            println!("{}", e);
            return;
        }
    };
}

fn write(file: &mut std::fs::File, source: &str) -> Result<(), ()> {
    match file.write(&source.trim().as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
