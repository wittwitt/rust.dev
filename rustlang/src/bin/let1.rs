struct Cat {
    age: i32,
}

// impl Drop for Cat {
//     fn drop(&mut self) {
//         println!("cat drop")
//     }
// }

fn main() {
    {
        let cat0 = &Cat { age: 3 }; //  *const pointer
        let cat00: *const Cat = &Cat { age: 3 };

        let cat2 = &mut Cat { age: 3 }; // &mut Cat，， raw pointer
        let cat22: *mut Cat = &mut Cat { age: 3 };
        cat2.age = 10;
        unsafe {
            (*cat22).age = 10;
        }

        let mut cat3 = &mut Cat { age: 3 };
        cat3.age = 10;
        // cat3 = &mut Cat { age: 11 };
        // cat3.age = 20;
    }

    {
        let mut cat = Cat { age: 3 };
        cat.age = 30;
        //
        cat = Cat { age: 4 };
        cat.age = 40;
    }

    println!("==== 1");
    {
        let mut cat1 = &mut Cat { age: 1 };
        println!("{}", cat1.age);

        let mut cat3 = Cat { age: 3 };
        cat1 = &mut cat3;
        println!("{}", cat1.age);
        println!("{}", cat3.age);

        // cat3.age = 10;
        // println!("{}", cat1.age);
        // println!("{}", cat3.age);
    }
    println!("==== 2");
    // {
    //     let cat = Pool { x: 3 };
    //     cat.x = 10;
    // }

    {
        let mut cat = Cat { age: 3 };
        cat.age = 10;

        cat = Cat { age: 11 };
        println!("cat{:?}", cat.age)
    }

    // {
    //     let   cat =  mut cat { x: 3 };
    //     cat.x = 10;
    // }

    // {
    //     let mut cat = &cat { x: 3 };
    //     cat.x = 10;
    // }

    {
        let ref mut cat = Cat { age: 3 };
        cat.age = 10;

        // cat = Pool { x: 11 };
    }

    {
        // let ref mut cat3: cat;
        // cat3 = &mut cat { x: 3 };

        let ref mut cat4 = &mut Cat { age: 1 };
        println!("{}", cat4.age);

        // let mut ref  cat: Pool;

        // let p = &mut Pool { x: 11 };
        // cat = p;

        // let mut c = &Pool { x: 3 };
        // c.x = 10;
        // c = &Pool { x: 10 };

        // let mut p1 = Pool { x: 3 };
        // cat = &p1;

        // let cat = &mut Pool { x: 3 };

        // println!("{}", cat3.x);
    }

    {
        let ref cat: Cat;
        // let cat: &Pool;
        cat = &Cat { age: 5 };
        // cat = &Pool { x: 10 };
        // cat.x = 6;// error
        // cat = &Pool { x: 10 }; // error

        println!("{}", cat.age);
    }
}

pub mod t1 {

    use super::Cat;

    pub fn f1() {
        let cat = Cat { age: 3 };

        // cat.age = 4; //不能修改字段
        // cat = Cat { age: 5 }; // 不能绑定其它

        println!("cat {}", cat.age);

        // let mut cat2 = Cat { age: 3 };
        let mut cat2: Cat = Cat { age: 3 };
        cat2.age = 4;
        println!("cat2 {}", cat2.age);

        cat2 = Cat { age: 5 };
        println!("cat2 {}", cat2.age);

        // let cat3 = &mut Cat { age: 3 };
        let cat3: &mut Cat = &mut Cat { age: 3 };
        cat3.age = 4;
        println!("cat3 {}", cat3.age);

        // cat3 = &mut Cat { age: 5 }// 不能绑定其它

        // let cat =mut cat { age: 3 }; // mut Cat 没有这个用法

        let cat4 = &Cat { age: 3 };
        // cat4.age = 4;
    }

    #[test]
    fn t1() {
        f1();
    }
}

pub mod t2 {
    pub fn f1(s: &mut String) {
        s.extend(",world".chars());
    }

    #[test]
    fn t1() {
        let mut s = "hello".to_owned();
        f1(&mut s);
        println!("{}", s);
    }
}
