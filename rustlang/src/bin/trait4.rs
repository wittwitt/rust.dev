trait T1 {
    fn say(&self) {}
}

trait T2 {
    fn say(&self) {}
}

struct Pool {}
impl T1 for Pool {
    fn say(&self) {
        println!("t1 say")
    }
}

impl T2 for Pool {
    fn say(&self) {
        println!("t2 say")
    }
}

fn main() {
    let p = Pool {};

    // p.say();
    T1::say(&p);
}
