// use std::sync::{Arc, RwLock};

// #[derive(Default)]
// struct Config {
//     pub debug_mode: bool,
// }

// impl Config {
//     pub fn current() -> Arc<Config> {
//         CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
//     }
//     pub fn make_current(self) {
//         CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
//     }
// }

// thread_local! {
//     static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
// }

// fn som() {
//     Config { debug_mode: true }.make_current();
//     if Config::current().debug_mode {
//         // do something
//         println!("test")
//     }
// }

struct Cat(Vec<u8>);

static mut CAT: Cat = Cat(Vec::new());

fn main() {
    unsafe {
        CAT.0.push(1);
        CAT.0.push(2);
        println!("{:?}", CAT.0);
    }

    unsafe {
        cc();
    }
}

unsafe fn cc() {
    CAT.0.push(1);
    CAT.0.push(2);
    println!("{:?}", CAT.0);
}

mod t1 {

    struct Cat(Option<Vec<u8>>);

    static mut CAT: Cat = Cat(Option::None);

    #[test]
    pub fn t1() {
        unsafe {
            if CAT.0.is_none() {
                CAT.0 = Some(Vec::new());
            }
        }
    }
}

// pub mod t2 {

//     use std::fmt::Debug;
//     use std::io;
//     use std::io::BufReader;
//     use std::io::Cursor;
//     use std::sync::Arc;

//     use anyhow::Result;
//     use hwloc::CpuSet;
//     use jsonrpc_core::futures_util::SinkExt;
//     use log::*;
//     use mapr::{Mmap, MmapMut, MmapOptions};

//     use once_cell::sync::OnceCell;

//     use lazy_static::lazy_static;

//     pub fn allocate_layer(sector_size: usize) -> Result<MmapMut> {
//         match MmapOptions::new()
//             .len(sector_size)
//             .private()
//             .clone()
//             .lock()
//             .map_anon()
//             .and_then(|mut layer| {
//                 layer.mlock()?;
//                 Ok(layer)
//             }) {
//             Ok(layer) => Ok(layer),
//             Err(err) => {
//                 // fallback to not locked if permissions are not available
//                 warn!("failed to lock map {:?}, falling back", err);
//                 let layer = MmapOptions::new().len(sector_size).private().map_anon()?;
//                 Ok(layer)
//             }
//         }
//     }
//     use once_cell::sync::Lazy;
//     use std::{collections::HashMap, sync::Mutex};

//     static mut SECTOR_SIZE: usize = 0;
//     lazy_static! {
//         static ref EXP_LABELS: Mutex<MmapMut> = {
//             unsafe {
//                 let mut mm = allocate_layer(SECTOR_SIZE).unwrap();
//                 Mutex::new(mm)
//             }
//         };
//         static ref LAYER_LABELS: Mutex<MmapMut> = {
//             unsafe {
//                 let mut mm = allocate_layer(SECTOR_SIZE).unwrap();
//                 Mutex::new(mm)
//             }
//         };
//     }

//     use std::cell::UnsafeCell;
//     use std::fs::File;

//     #[test]
//     pub fn t1() {
//         // setup_create_label_memory().unwrap();

//         unsafe {
//             SECTOR_SIZE = 1000;
//         }

//         let mut ff = LAYER_LABELS.lock().unwrap();

//         read_layer(&mut ff).unwrap();
//     }

//     pub fn read_layer(mut data: &mut [u8]) -> Result<()> {
//         let data_path = "/home/sh/rust.dev/src/bin/static2.rs";
//         let file = File::open(data_path).unwrap();
//         let mut buffered = BufReader::new(file);

//         io::copy(&mut buffered, &mut data).unwrap();

//         Ok(())
//     }
// }

pub mod t3 {

    use std::sync::Mutex;

    use lazy_static::lazy_static;

    pub struct Cat {
        age: i32,
    }

    pub fn sa(cat: &mut Cat) {
        println!("sa {}", cat.age);
        cat.age = 1000;
    }

    static mut CAT: Cat = Cat { age: 100 };

    lazy_static! {
        static ref CAT2: Mutex<Cat> = {
            let mut mm = Cat { age: 100 };
            mm.age = 200;
            Mutex::new(mm)
        };
    }

    pub fn f1() {
        //
        unsafe {
            sa(&mut CAT);
            println!("t1 cat: {}", CAT.age);
        }

        let mut cat2 = CAT2.lock().unwrap();
        sa(&mut cat2);
        println!("t1 cat2: {}", cat2.age);
    }

    #[test]
    pub fn t1() {
        f1();
    }
}
