use std::thread::Thread;

use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Config> = OnceCell::new();

struct Config {
    wdpost_try: i32,
    wdpost_port: &'static str,
    scfs_path_prefix: &'static str,
}

fn main() {
    if CONFIG.get().is_none() {
        CONFIG.set(Config {
            wdpost_try: 10,
            wdpost_port: "abc",
            scfs_path_prefix: "/scfs",
        });
    }

    let mut ts = Vec::new();
    for _ in 1..10 {
        let h = std::thread::spawn(|| {
            println!("{}", CONFIG.get().unwrap().wdpost_try);
        });
        ts.push(h);
    }

    for h in ts {
        h.join();
    }
}
