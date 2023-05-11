use std::{thread::sleep, time::Duration};

use flexi_logger;
use log::info;

mod lock;

fn main() {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    loop {
        doing();
        info!("write ok");
        sleep(Duration::from_secs(3));
    }
}

fn doing() {
    let _f_lock = lock::WriteLayerDLock::lock();
    sleep(Duration::from_secs(3));
}
