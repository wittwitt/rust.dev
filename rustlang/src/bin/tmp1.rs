// fn main() {
//     let size = 2_i64.pow(34) as usize;

//     let mut vec = Vec::with_capacity(size);

//     // let mut vec = Vec::new();

//     // this may fail, but won't hard abort
//     // vec.try_reserve(size).expect("h");

//     // now you can safely add up to the `size` to the vec
//     // vec.resize(size, -1.0);

//     vec.push(1);
// }

// use memmap; // 0.7.0

use core::time;

use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::Result;
use log::*;

use mapr::{Mmap, MmapMut, MmapOptions};

// const SIZE: u64 = 1 * 1024 * 1024 * 1024;

const SIZE: u64 = 1 * 1024;

fn main() {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();
    info!("start");

    f3();
}

#[allow(dead_code)]
fn f1() {
    let src = "Hello!";

    let mut data = MmapOptions::new()
        .len(SIZE as usize)
        .private()
        .clone()
        .lock()
        .map_anon()
        .and_then(|mut layer| {
            layer.mlock()?;
            Ok(layer)
        })
        .unwrap();

    std::thread::sleep(time::Duration::from_secs(10));
    data[..src.len()].copy_from_slice(src.as_bytes());
}

#[allow(dead_code)]
fn f2() {
    let src = "Hello!dddddddddddddddddd";

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/tmp/test.mmap")
        .expect("Unable to open file");

    // Allocate space in the file first
    f.seek(SeekFrom::Start(SIZE)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let mut data = unsafe {
        MmapOptions::new()
            .offset(0)
            .len(SIZE as usize)
            // .private()
            .lock()
            .map_mut(&f)
            .unwrap()
    };
    info!("--");
    // std::thread::sleep(time::Duration::from_secs(10));
    data[..src.len()].copy_from_slice(src.as_bytes());
    info!("--");
    // std::thread::sleep(time::Duration::from_secs(10));
}

#[allow(dead_code)]
fn f3() {
    let path = "/tmp/test.mmap";
    let mut mm = allocate_layer_labels(SIZE as usize, PathBuf::from(path)).unwrap();

    let src = "this is mmap";
    info!("--");
    mm[..src.len()].copy_from_slice(src.as_bytes());
    info!("--");
}

pub fn allocate_layer_labels(sector_size: usize, path: PathBuf) -> Result<MmapMut> {
    info!("==allocate_layer_labels: {:?}", path);

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Unable to open file");

    f.seek(SeekFrom::Start(sector_size as u64)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    unsafe {
        match MmapOptions::new()
            .offset(0)
            .len(sector_size)
            .clone()
            .lock()
            .map_mut(&f)
            .and_then(|mut layer| {
                layer.mlock()?;
                Ok(layer)
            }) {
            Ok(layer) => Ok(layer),
            Err(err) => {
                // fallback to not locked if permissions are not available
                warn!("failed to lock map {:?}, falling back", err);
                let layer = MmapOptions::new().len(sector_size).private().map_anon()?;
                Ok(layer)
            }
        }
    }
}
