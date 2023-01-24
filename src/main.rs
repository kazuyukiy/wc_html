mod thread_pool;
// mod wc_handler5;
// mod swimming;

fn main() {
    // println!("Hello, world!");

    //    wc_handler5::system_ini();
    let pool = thread_pool::ThreadPool::new(4);
}
