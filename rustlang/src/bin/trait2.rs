trait Animal {
    // fn eat(&self) -> &Self;
    fn eat(&self);
}

struct Horse {}

impl Animal for Horse {
    // fn eat(self: &Self) -> &Self {
    //     println!("horse eat");
    //     self
    // }
    fn eat(self: &Self) {
        println!("horse eat");
    }
}

struct Monkey {}

impl Animal for Monkey {
    // fn eat(self: &Self) -> &Self {
    //     println!("monkey eat");
    //     self
    // }
    fn eat(self: &Self) {
        println!("monkey eat");
    }
}

// 普通范型，静态替换，调用者必须指定具体的类型
fn f1<T: Animal>(n: T) -> T {
    n.eat();
    n
}

fn f11<T: Animal>(n: &T) -> &T {
    n.eat();
    n
}

// 参数，和范型作用类似，但返回的具体类型可以由函数决定（只有一个某种类型的时候）
// impl Animal 为静态分发
fn f2(n: impl Animal) -> impl Animal {
    n.eat();

    let m = Monkey {};
    m
}

// 提示 Box<dyn Animal> ，，，当有多种可能的时候，
// dny Animal 为动态分发
// fn f22(n: impl Animal, arg2: i32) -> impl Animal {
// fn f22(n: impl Animal, arg2: i32) -> Box<dyn Animal> {
// fn f22<T: Animal>(n: T, arg2: i32) -> Box<dyn Animal>
// where
//     T: 'static,
// {
fn f22(n: impl Animal + 'static, arg2: i32) -> Box<dyn Animal> {
    n.eat();
    if arg2 > 1 {
        let m = Monkey {};
        return Box::new(m);
    }
    Box::new(n)
}

fn main() {
    println!("impl Trait");

    {
        let h = Horse {};
        f1(h);
        // h.eat(); // error move

        let h2 = Horse {};
        f11(&h2);
        h2.eat();

        let m = Monkey {};
        f1(m);
    }
    println!("=== === \n\n");

    {
        let h = Horse {};
        let h2 = f2(h);
        h2.eat();
        let h3 = f22(h2, 3);
        h3.eat();

        println!("--- ---");

        let m = Monkey {};
        let m2 = f2(m);
        m2.eat();
    }
}

// rust trait 边界 ???/
