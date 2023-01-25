use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod thread_pool;
mod wc_handler;
// mod swimming;

// pub mod page;

mod page;
mod shop;
mod world;

fn main() {
    // println!("Hello, world!");

    // DBG
    world::hello();
    // world::hello_from_japan();
    // world::hello_from_koria();
    // world::hello_from_koria();
    // world::koria::call_japan();

    // DBG
    // shop::shop();

    wc_handler::system_ini();

    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(v) => v,
        Err(_) => return,
    };

    let pool = thread_pool::ThreadPool::new(4);

    for stream in listener.incoming() {
        // let stream = stream.unwrap();
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // println!("main.rs fn handle_connection");

    // Consider to reject access from wher not local
    let response = wc_handler::response(&mut stream);
    stream.write(&response).unwrap();
    stream.flush().unwrap();
} // end of fn handle_connection
