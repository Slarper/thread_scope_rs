use std::{thread::sleep, time::Duration};

fn main() {
    use std::thread;

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..10 {
                sleep(Duration::from_millis(100));
                println!("hello from a scoped thread: {}", i);
            }
        });

        s.spawn(|| {
            for i in 0..5 {
                sleep(Duration::from_millis(200));
                println!("hello from another scoped thread: {}", i);
            }
        });
    });

    println!("Hello, world!");
}
