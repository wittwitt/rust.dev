use std::env;
use std::env::temp_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::io::{Seek, SeekFrom};

use mapr::MmapOptions;

use mapr::Mmap;

/// Output a file's contents to stdout. The file path must be provided as the first process
/// argument.
fn main() {
    f2();
}

#[allow(dead_code)]
fn f2() {
    let path = "/tmp/test.mmap";

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(13).unwrap();

    let mut mmap = unsafe {
        MmapOptions::new()
            .lock() //
            .map_mut(&file)
            .unwrap()
    };

    mmap.copy_from_slice(b"Hello, world!");
}

#[allow(dead_code)]
fn f1() {
    let path = "/tmp/test.mmap";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Unable to open file");

    file.seek(SeekFrom::Start(100)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    io::stdout()
        .write_all(&mmap[..])
        .expect("failed to output the file contents");
}
