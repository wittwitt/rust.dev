#[path = "b.rs"]
mod b;

// mod super::b;
// mod b;
#[allow(dead_code)]
pub fn say() {
    b::say();

    println!("mod3 a say")
}

mod q {
    #[path = "other.rs"] // a/q/other
    mod inner;
    #[allow(dead_code)]
    fn say() {
        inner::say();
    }
}
