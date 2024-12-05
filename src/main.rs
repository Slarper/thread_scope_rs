use std::{thread::sleep, time::Duration};

fn main() {
    use std::thread;

    let x = thread::scope(|s| {
        s.spawn(|| {
            for i in 0..10 {
                sleep(Duration::from_millis(100));
                println!("hello from a scoped thread: {}", i);
            }
        });

        let k = s.spawn(|| {
            for i in 0..5 {
                sleep(Duration::from_millis(100));
                println!("hello from another scoped thread: {}", i);
            }
            114514
        });

        

        let b = s.spawn(|| {
            println!("hello 111");
            1919810
        });


        let m = k.join().unwrap();
        println!("m: {}", m);
        let n = b.join().unwrap();
        println!("n: {}", n);

        (m,n)
    });

    println!("result: {}", x.0 + x.1);

    println!("Hello, world!");
}
