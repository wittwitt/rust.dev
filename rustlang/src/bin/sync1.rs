fn main() {}

pub mod t1 {
    use std::sync::{Arc, Mutex};
    use std::thread::{self, JoinHandle};

    const N_TIMES: u64 = 1000;
    const N_THREADS: usize = 10;

    static mut R: u64 = 0;

    fn reset() {
        unsafe {
            R = 0;
        }
    }

    fn add_n_times(n: u64, mutex: Arc<Mutex<()>>) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                let lock = mutex.lock().unwrap();
                // critical section START
                unsafe {
                    R += 1;
                }
                // critical section END
                drop(lock);
            }
        })
    }

    pub fn f1() {
        let mutex = Arc::new(Mutex::new(()));

        loop {
            reset();

            let mut threads = Vec::with_capacity(N_THREADS);

            for _ in 0..N_THREADS {
                threads.push(add_n_times(N_TIMES, mutex.clone()));
            }

            for thread in threads {
                thread.join().unwrap();
            }

            assert_eq!(N_TIMES * N_THREADS as u64, unsafe { R });
        }
    }

    #[test]
    fn t1() {
        f1();
    }
}

pub mod t2 {

    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    struct Counter {
        count: u64,
    }

    pub fn f1() {
        let n = Mutex::new(Counter { count: 0 });

        n.lock().unwrap().count += 1;

        let n = AtomicU64::new(0);

        n.fetch_add(0, Ordering::Relaxed);
    }

    #[test]
    fn t1() {
        f1();
    }
}

pub mod t3 {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

    pub fn f1() {
        for _ in 0..100 {
            PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);
        }

        println!("Packets served = {:?}", PACKETS_SERVED);
    }

    #[test]
    fn t1() {
        f1();
    }
}

pub mod t4 {

    // use std::collections::HashMap;

    // use std::lazy::SyncLazy;

    // static HASHMAP: SyncLazy<HashMap<i32, String>> = SyncLazy::new(|| {
    //     println!("initializing");
    //     let mut m = HashMap::new();
    //     m.insert(13, "Spica".to_string());
    //     m.insert(74, "Hoyten".to_string());
    //     m
    // });

    // pub fn f1() {
    //     println!("ready");
    //     std::thread::spawn(|| {
    //         println!("{:?}", HASHMAP.get(&13));
    //     })
    //     .join()
    //     .unwrap();
    //     println!("{:?}", HASHMAP.get(&74));
    // }

    // #[test]
    // fn t1() {
    //     f1();
    // }
}
