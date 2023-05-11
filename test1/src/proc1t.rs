use proc1::say_hi;

#[say_hi]
fn some() {
    println!("this is some method");
}

pub fn t1() {
    dummy();

    f2();
}

mod t {
    #[test]
    fn t1() {
        super::t1()
    }
}
