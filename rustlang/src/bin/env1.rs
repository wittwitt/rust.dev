use std::time::Duration;

use tempfile::tempdir;

fn main() {
    println!("system temp dir: {:?}", std::env::temp_dir());
    let temp_dir = tempdir().expect("tempdir failure");

    println!("temp_dir( /path-to-tmp/.XXXX ), drop auto: {:?}", temp_dir);
    std::thread::sleep(Duration::from_secs(30));
}
