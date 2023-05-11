use std::fmt::Display;

struct Dog<T> {
    name: &'static str,
    so: T,
}

// impl DogStrTr&ait for Dog<String> {
//     fn say(&self) {}
// }

impl<T: Display> Dog<T> {
    #[allow(dead_code)]
    fn say(&self) {
        println!("{}, say: {}", self.name, self.so);
    }
}

#[test]
fn t1() {
    let d = Dog { name: "zs", so: 1 };
    d.say()
}
