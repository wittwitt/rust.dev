// use core::sync;
use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    // sync::Arc,
};

// static CONFIG: RefCell<String> = RefCell::new();

// static CONFIG: Arc<Dinosaur> = Arc::new(Dinosaur::new());

fn main() {
    refcell_1();
}

struct Dinosaur {
    x: Cell<i32>,
}

impl Dinosaur {}

// （1）Cell 是操作T(values), RefCell操作&T(references)
// （2）Cell 在编译器检查，运行时不会panic；RefCell在运行时检查，使用不当会发生panic

#[derive(Copy, Clone)]
struct Dog {
    // name: String,
    age: i32,
}

struct Cat {
    age: i32,
}

struct Maziwei {
    c: RefCell<Cat>,
}

struct Hongcs {
    c: Cat,
}

#[allow(dead_code)]
fn refcell_1() {
    {
        let mzw = Maziwei {
            c: RefCell::new(Cat { age: 10 }),
        };
        println!("{}", mzw.c.borrow().age);

        mzw.c.replace(Cat { age: 100 });
        println!("{}", mzw.c.borrow().age);

        mzw.c.borrow_mut().age = 2;
        println!("{}", mzw.c.borrow().age);
    }

    {
        let mut mzw = Hongcs { c: Cat { age: 10 } };
        println!("{}", mzw.c.age);

        mzw.c = Cat { age: 100 };
        println!("{}", mzw.c.age);

        mzw.c.age = 2;
        println!("{}", mzw.c.age);
    }
}

#[allow(dead_code)]
fn t2() {
    {
        let d = Dinosaur { x: Cell::new(0) };
        d.x.set(100);
        println!("x: {}", d.x.get());

        let x = Cell::new(0);
        x.set(100);
        println!("x: {}", x.get());

        x.set(x.get() + 100);
        println!("x: {}", x.get());
    }

    {
        let d = Cell::new(Dinosaur { x: Cell::new(1) });
        let d2 = d.borrow(); //  Copy trait
                             // d2.x.set(10);
                             // println!("d: {:?}", d.get_mut().x);
                             // println!("d2: {}", d2.x.get());
    }
    //
    {
        let c = RefCell::new("abc".to_string());
        c.borrow_mut().push_str("string");
        println!("{}", c.borrow())
    }
    //
    {
        let mut c = RefCell::new("abc".to_string());
        // c.borrow_mut().push_str("string2");
        c.try_borrow_mut().unwrap().push_str("string2");
        c.get_mut().push_str("string");
        println!("{}", c.get_mut());
    }

    {
        let mut d = Cell::new(Dog {
            // name: "".to_string(),
            age: 10,
        });
        d.get_mut().age = 111;
        println!("age: {}", d.get().age);

        let mut dd = d.get();
        dd.age = 200;

        println!("age: {}", d.get().age);
        println!("age: {}", dd.age);
    }

    {
        let d = Cell::new(Dog {
            // name: "".to_string(),
            age: 10,
        });

        let mut dd = d.get(); // dd not d ,,,is copy of d
        dd.age = 100;

        println!("age: {}", d.get().age);
    }

    let mut dc: &str = "abc";
    println!("{}", dc);
    dc = "ggg";
    println!("{}", dc);

    let mut dc2 = "abc";
    println!("{}", dc2);
    dc2 = "abcd";
    println!("{}", dc2);
}
