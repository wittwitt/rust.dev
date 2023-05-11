use std::{sync::Arc, sync::Mutex, thread};

fn main() {
    let cat = Arc::new(Mutex::new(Cat {
        name: "cat1".to_string(),
    }));

    let mut hi = "hi".to_string();
    let hi_arc = Arc::new(Mutex::new(hi));

    let mut runner_list = Vec::new();
    for item in 0..10 {
        let hi = hi_arc.clone();
        let cat = cat.clone();
        let runner = thread::spawn(move || {
            let mut cat_lock = cat.lock().unwrap();

            let mut hi_lock = hi.lock().unwrap();

            let hi_lock = format!("{},{},nihao", item, hi_lock);

            cat_lock.name = "abc".to_string();

            println!("{:?}, {:?}", hi_lock, cat_lock.name);
        });
        runner_list.push(runner);
    }

    for runner in runner_list.into_iter() {
        runner.join().unwrap();
    }
}

struct Cat {
    name: String,
}

pub mod t {
    use std::{
        thread::{sleep, spawn},
        time::{Duration, SystemTime},
    };

    pub fn ff1() {
        let h1 = spawn(|| loop {
            println!("h1: {:?}", SystemTime::now());
            sleep(Duration::from_secs(2));
        });

        let h2 = spawn(|| loop {
            println!("h2: {:?}", SystemTime::now());
            sleep(Duration::from_secs(2));
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }

    #[test]
    fn f1() {
        ff1();

        loop {}
    }
}
