use std::sync::{Arc, Mutex};
use std::thread;

struct Cat {
    age: i32,
}

fn main() {
    let mut c = Arc::new(Mutex::new(Cat { age: 10 }));

    let c1 = c.clone();

    let mut n = c.lock().unwrap();
}

pub fn main2() {
    let nums = Arc::new(Mutex::new(vec![]));

    let mut childs = vec![];
    for n in 0..5 {
        let ns = nums.clone();
        let c = thread::spawn(move || {
            let mut v = ns.lock().unwrap();
            v.push(n);
        });
        childs.push(c);
    }
    for c in childs {
        c.join().unwrap();
    }
    println!("{:?}", nums);
}
