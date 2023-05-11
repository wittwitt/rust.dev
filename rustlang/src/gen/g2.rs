use std::fmt::Display;

#[test]
fn t1() {
    let d = Dog {
        x: &String::from("abc"),
    };

    d.say();
}

struct Dog<'a, T: 'a + Display> {
    x: &'a T,
}

impl<'a, T: Display> Dog<'a, T> {
    #[allow(dead_code)]
    fn say(&self) {
        println!("{}", self.x)
    }
}
