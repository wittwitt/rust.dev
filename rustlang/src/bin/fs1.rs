use std::{fs::OpenOptions, io::Write};

fn main() {
    let out_path = "./c.txt";

    let mut f_data = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&out_path)
        .unwrap();

    f_data.write("abc".as_bytes()).unwrap();

    // Zero-pad the data to the requested size by extending the underlying file if needed.
    f_data.set_len(1024 as u64).unwrap();
    f_data.write("abc".as_bytes()).unwrap();
}
