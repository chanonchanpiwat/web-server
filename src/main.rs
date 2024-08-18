use std::{thread, time::{Duration, Instant}};
use web_server::threadpool::ThreadPool;


fn main() {
    let start = Instant::now();

    let cores = std::thread::available_parallelism().unwrap();
    
    let pools = ThreadPool::new(cores.into());

    let sleep = || {
        println!("Sleep 5 s");
        thread::sleep(Duration::from_secs(5));
        println!("Sleep finished");
    };

    for _ in 0..28 {
        pools.execute(sleep);
    }


    drop(pools);

    println!("time taken :{:?}", start.elapsed())
}
