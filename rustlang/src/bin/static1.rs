use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Mutex},
};

use once_cell::sync::OnceCell;

static SOME: Dog = Dog::new(100);

static mut DOG2: Dog = Dog::new(200);

// static S2: Box = Box::new();

// static S3: Box<String> = Box::new("".to_string());

static S2: OnceCell<String> = OnceCell::new();

static mut II: i32 = 11;
// static II2: Cell<i32> = Cell::new(0);
// static mut II3: Box<i32> = Box::new(33);

static SS: &'static str = "babc";
// static SS2: str = "babc";

// static S3: RefCell<String> = RefCell::new("value".to_string());

// static S3: Arc<Mutex<Box<String>>> = Arc::new(Mutex::from(Box::new("abc".to_string())));

struct Dog(i32);

impl Dog {
    const fn new(i: i32) -> Self {
        Dog(i)
    }
}

struct Cat {
    x: Box<String>,
}
// impl Cat {
//     const fn new() -> Self {
//         Cat {
//             x: Box::new("abc".to_string()),
//         }
//     }
// }

fn main() {
    unsafe {
        II = 300;
        println!("{}", II);
    }

    let c = Cell::new(1);

    unsafe {
        DOG2.0 = 1909;
        println!("{}", DOG2.0);

        c.set(DOG2.0);
    }

    println!("{}", c.get());

    unsafe { DOG2.0 = 999 }
    println!("{}", c.get());

    S2.set("abc".to_string()).unwrap();

    println!("{:?}", SOME.0);

    println!("{}", S2.get().unwrap());

    // let s = "abc".to_string();
    // s.push_str("abc");
}
