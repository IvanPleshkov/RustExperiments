use std::thread;

fn main() {
    let res = thread::spawn(move || {
        let mut v : Vec<Vec<u8>> = Vec::new();
        loop {
            v.push(Vec::with_capacity(100_000_000_000))
        }
    }).join();
    if let Err(e) = res {
        println!("Thread paniced!");
    }
}
