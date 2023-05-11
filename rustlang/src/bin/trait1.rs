trait Animal {
    fn m1(self: Self); // 第一个参数，隐含类型Self
    fn m2(self: &Self);
    fn m3(self: &mut Self);

    // 简写
    fn m11(self);
    fn m22(&self);
    fn m33(&mut self);
}

trait Eat {
    fn eat(self: &Self);
    fn eat2(self: Self);
}

struct Horse {
    name: String,
}

impl Eat for Horse {
    fn eat(self: &Self) {
        println!("{} is eating", self.name);
    }

    fn eat2(self: Self) {}
}

fn main() {
    let h = Horse {
        name: String::from("haha"),
    };
    h.eat();
}
