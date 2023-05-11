fn main() {}

mod t1 {

    struct Cat {
        age: i32,
    }

    #[test]
    fn t1() {
        let mut cat = Cat { age: 10 };
        say(&mut cat);
        println!("t1: {}", cat.age)
    }

    fn say(cat: &mut Cat) {
        cat.age = 20;
        println!("say: {}", cat.age)
    }
}

#[test]
fn t1() {
    println!("{}", say1(&10));

    // say(&10);
}

// fn say<'a>(i: &'a i32) -> &'a str {
//     let s = format!("{}", i);
//     &s
// }

pub fn say1(i: &i32) -> String {
    format!("{}", i)
}

#[test]
fn t2() {
    say2("a", "b");
}

pub fn say2(s: &str, s2: &str) {
    println!("{},,{}", s, s2);
}

/*
#[test]
fn t3() {
    let d;
    let a = "ab";
    {
        let b = "cd";
        // println!("{}", say3(&a, &a));

        d = say3(&a, &b);
        println!("{}", d);
    }
} */

/*
#[test]
fn t4() {
    let a = String::from("ab");
    let d;
    {
        let b = String::from("cd");
        d = say3(&a, &b);
        println!("{}", d);
    }
    println!("{}", d);
}

pub fn say3<'a>(s: &'a str, s2: &'a str) -> &'a str {
    if s.len() > 2 {
        s2
    } else {
        s
    }
}
*/

struct Cat {
    name: String,
}
impl Drop for Cat {
    fn drop(&mut self) {
        println!("cat {} drop", self.name)
    }
}

#[test]
fn t5() {
    let _c1 = Cat {
        name: String::from("c1"),
    };

    println!("c1");

    let _c2 = Cat {
        name: String::from("c2"),
    };

    println!("c2");
}

#[test]
fn t6() {
    {
        let mut _cw = Cat {
            name: String::from("cw"),
        };
    } // drop _cw

    let mut _c1 = Cat {
        name: String::from("c1"),
    };

    println!("c1");

    let _c1 = Cat {
        name: String::from("c2"),
    };

    println!("c2");

    // c1
    // c2
    // cat c2 drop
    // cat c1 drop
}

#[test]
fn t7() {
    let mut x;
    // {
    let y = String::from("ab");
    x = &y;
    // }
    println!("{}", x);
}

#[test]
fn t8() {
    t8foo();
}

fn t8foo() {
    let x = 520; //本来我希望caller调用这个520
    {
        let x = 1314; //不小心，我在一个子域中 多写了这么一句
        let out = caller(x);
        println!("---> {} <---", out); //此时我们引入了一个错误，但编译器并没有发现
    }
}

fn caller(i: i32) -> i32 {
    10
}

#[test]
fn t9() {
    let info = String::from("File not found.");
    // 存放结果值的变量
    let exc = ImportantExcepiton {
        part: info.as_str(),
    };

    println!("{:?}", exc);
}

#[derive(Debug)]
struct ImportantExcepiton<'a> {
    part: &'a str,
}

#[test]
fn t10() {
    let s = get_str();
    println!("s: {}", s);

    let s2 = get_str2();
    println!("s2: {}", s2);

    let i1 = get_i32();
    println!("i1: {}", i1);

    // let c = get_cat();
    // println!("{}", c.name);
}

pub fn get_str<'a>() -> &'a str {
    "get_str" // 'static
}

pub struct Dog<'a>(i32, &'a str);

const DOG: Dog = Dog(10, "abc");

pub fn get_str2<'a>() -> &'a str {
    let a = "get_str2"; // const
    a // 'static
}

pub fn get_i32<'a>() -> &'a i32 {
    &10
}

// fn get_cat<'a>() -> &'a Cat {
//     &Cat {
//         name: String::from("mzw"),
//     }
// }

pub fn get_dog<'a>() -> &'a Dog<'a> {
    &DOG
}

// pub fn get_slie<'a>() -> &'a Cat {
//     unsafe {
//         &Cat {
//             name: String::from(""),
//         }
//     }
// }
