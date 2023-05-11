mod t1 {
    #[allow(dead_code)]
    pub static ENABLE: bool = false;
}

#[test]
fn t1() {
    // t1::ENABLE = true;

    println!("{}", t1::ENABLE);
}
