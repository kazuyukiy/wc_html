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
    let pool = thread_pool::ThreadPool::new(4);
}
