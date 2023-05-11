use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    match say() {
        Ok(s) => {
            println!("ok: {}", s);
        }
        Err(e) => {
            println!("err: {}", e);
            println!("err: {:?}", e);
        }
    }
}

fn say() -> Result<String, CError> {
    if true {
        return Err(CError {});
    }

    Ok(String::from("okok"))
}

// #[derive(Debug)]
struct CError {}

impl Debug for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is debug")
    }
}

impl Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "abc")
    }
}

impl Error for CError {}
