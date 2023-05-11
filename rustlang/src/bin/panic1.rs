use std::panic;

fn main() {
    say();

    println!("continue======")
}

fn say() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
        3
    });
    assert!(result.is_ok());

    println!("{:?}", result.unwrap());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
}
