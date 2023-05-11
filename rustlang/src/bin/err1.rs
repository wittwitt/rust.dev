// use std::backtrace::Backtrace;
use std::convert::From;
use std::fmt;
use std::fs;
use std::io;

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

fn main() {
    // a1();

    // a2(1);
    // a2(11);

    // a3(1);
    // a3(11);

    a4();
}

/// a1 使用自定义类型，如果需要传递，需要实现对用的from，将一种Error转化为最终返回的Error
///
#[allow(dead_code)]
fn a1() {
    let b1_result = b1();

    match b1_result {
        Ok(ref msg) => {
            println!("{}", msg)
        }
        Err(e) => {
            println!("err: {}", e)
        }
    }
}

fn b1() -> Result<String, MyError> {
    let c = c1()?; // MyError::from()

    // ?
    // let c = sss();
    // if let Err(e) = c {
    //     return Err(MyError::from(e));
    // }

    Ok(format!("c: {}", c))
}

fn c1() -> Result<String, String> {
    Err(String::from("this is c1 error"))
}

#[derive(Debug)]
pub enum MyError {
    BadSchema(String, String, String),
    IO(io::Error),
    Read,
    Receive,
    Send,
    Some(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::BadSchema(s1, s2, s3) => {
                write!(f, "BadSchema Error:{}, {}, {}", s1, s2, s3)
            }
            MyError::IO(e) => {
                write!(f, "IO Error: {}", e)
            }
            MyError::Read => {
                write!(f, "Read Error")
            }
            MyError::Receive => {
                write!(f, "Receive Error")
            }
            MyError::Send => {
                write!(f, "Send Error")
            }
            MyError::Some(ref e) => {
                write!(f, "some.fmt: {}", e)
            }
        }
    }
}
impl From<String> for MyError {
    // my error 要实现对应的from
    fn from(str: String) -> MyError {
        MyError::Some(str)
    }
}

/// a2 使用box实现通用error类型传递
///
#[allow(dead_code)]
fn a2(i: i32) {
    let b_rs = b2(i);
    if let Err(err) = b_rs {
        println!("b2: {}", err)
    }
}

fn b2(i: i32) -> Result<String, Box<dyn std::error::Error>> {
    let c = c2(i)?;
    Ok(format!("b2: {}", c))
}

fn c2(i: i32) -> Result<String, String> {
    if i > 10 {
        Ok(String::from("this is c2 ok"))
    } else {
        Err(String::from("this is c2 error"))
    }
}

//
//
//

#[allow(dead_code)]
fn a3(i: i32) {
    let b3_rs = b3(i);
    if let Err(err) = b3_rs {
        println!("b3 err: {:#}, {:?}", err, err) // :# causes
    }
}

fn b3(i: i32) -> Result<String> {
    // let c3 = c3(i).context("c3haerr")?;
    let c3 = c3(i).with_context(|| format!("c3 err"))?;
    Ok(format!("c3 ok: {}", c3))
}

fn c3(i: i32) -> Result<String> {
    if i > 10 {
        println!("c3 ok: {}", i);
        Ok(String::from("this is c3 ok"))
    } else {
        println!("c3 err: {}", i);
        Err(anyhow!("this is c3 error: {}", i))
    }
}

//
//
//

#[derive(Error, Debug)]
pub enum SomeError {
    #[error("111 xxxx: {source:?}")]
    Io {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },
    // #[error("data store disconnected: {0:?}")]
    // Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[allow(dead_code)]
fn a4() {
    let b4 = b4();
    if let Err(err) = b4 {
        println!("{:#}", err)
    }
}

fn b4() -> Result<String, SomeError> {
    _ = c4()?;

    Err(SomeError::InvalidHeader {
        expected: String::from("exex"),
        found: String::from("fff"),
    })
}

fn c4() -> Result<String, io::Error> {
    let f = fs::OpenOptions::new().read(true).open("vec.rs")?;

    println!("fff{:?}", f);

    Ok(String::from("ok"))
}

fn a5() {}

fn b5() -> Result<String> {
    Err(SomeError::Redaction(String::from("haha")).into())
}
