use std::panic;

fn main() {}

#[test]
fn f1() {
    let result = match panic::catch_unwind(|| {
        panic!("ccccccc");
        3
    }) {
        Ok(t) => true,
        Err(panic) => {
            let error_msg = match panic.downcast_ref::<&'static str>() {
                Some(message) => message,
                _ => "no unwind information",
            };
            println!("vvvvvvvv{:?}", error_msg);
            true
        }
    };
}

#[test]
fn f2() {}
