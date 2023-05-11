#[allow(dead_code)]
struct Cat {
    x: i32,
}
impl Cat {
    #[allow(dead_code)]
    fn new() -> Self {
        Cat { x: 0 }
    }
    #[allow(dead_code)]
    fn add(&mut self, x: i32) {
        self.x = self.x + x;
    }
}

#[test]
fn t1() {
    let mut c = Cat::new();
    c.add(1);
}
