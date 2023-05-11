use std::collections::BTreeMap;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // let mut p = Pool { x: 0 };
    let m = Arc::new(Mutex::new(Pool::new()));

    let mut handles = Vec::new();
    for index in 1..10 {
        let m = m.clone();
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));

                let mut pp = m.lock().unwrap();

                pp.x = pp.x + 1;

                pp.m.insert(format!("inde: {},{}", index, i), "a");

                pp.c.push((format!("inde: {},{}", index, i), format!("{}", i)))
            }
        });
        handles.push(handle);
    }

    // for i in 1..20 {
    //     println!("====hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    for i in handles {
        i.join().unwrap();
    }

    let p = m.lock().unwrap();

    for (k, v) in p.m.iter() {
        println!("{}={}", k, v);
    }

    println!("=================c{}", p.x);

    for (k, v) in p.c.iter() {
        println!("{}={}", k, v);
    }

    println!("c{}", p.x);
}

struct Pool<T, V> {
    x: i32,
    m: BTreeMap<T, V>,
    c: Vec<(String, String)>,
}

impl<T, V> Pool<T, V> {
    fn new() -> Self {
        Pool {
            x: 0,
            m: BTreeMap::new(),
            c: Vec::new(),
        }
    }
}
