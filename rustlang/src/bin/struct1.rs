struct Horse {
    name: String,
    age: i32,
}

impl Horse {
    // fn eat(self: &Horse) {
    // fn eat(self) {
    fn eat(self: &Self) {
        println!("{} is eating", self.name);
    }

    fn eat2(self: Self) {
        println!("{} eat over!!!", self.name);
    }

    fn new(name: String, age: i32) -> Horse {
        Horse {
            name: name,
            age: age,
        }
    }

    // fn new(name: String, age: i32) -> &Horse { // 返回引用是不可以
    //     let h = Horse {
    //         name: name,
    //         age: age,
    //     };
    //     &h // 这样造成悬坠指针了
    // }
}

fn main() {
    let h = Horse {
        name: String::from("maziwei"),
        age: 10,
    };
    println!("name: {}, age: {}", h.name, h.age);

    {
        h.eat();
        h.eat2();
    }

    // {
    //     h.eat2();  // 一旦调用了，h就move了，后续 error
    //     h.eat();
    // }

    //
    let h2 = Horse::new(String::from("hongchaosheng"), 10);
    h2.eat();
}
