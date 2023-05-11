use std::fmt::{self, Debug};

struct Cat {
    age: i32,
}

fn main() {
    // f11();
    f22();
}

pub fn f11() {
    let mut cat = Cat { age: 5 };

    // let mut cat2 = cat;

    let cat2 = &mut cat;
    println!("{}", cat2.age);
    // f1(&mut cat2);

    f1(cat2);
    println!("{}", cat2.age);

    println!("{}", cat.age);
}

fn f22() {
    let cat = &mut Cat { age: 1 };

    cat.age = 3;

    f1(cat);
    println!("{}", cat.age);

    f1(cat);
    println!("{}", cat.age);
}

fn f1(cat: &mut Cat) {
    cat.age = cat.age + 1;
}

#[test]
fn t1() {
    f11();
}

pub fn f2(s: String) -> String {
    s.chars().rev().collect()
}

#[test]
fn t2() {
    let s1 = String::from("abcd");

    f2(s1); // s1 所有权 move
            // println!("{}", s1);
}

#[test]
fn t3() {
    let s1 = String::from("abcd");

    let my_value_ref = &s1;
    // convert the reference to a raw pointer
    let my_value_raw_ptr = my_value_ref as *const String;
    // convert the raw pointer to an integer
    let my_value_addr = my_value_raw_ptr as usize;

    println!("address = {:X}", my_value_addr);

    let s2 = s1;

    let my_value_ref = &s2;
    // convert the reference to a raw pointer
    let my_value_raw_ptr = my_value_ref as *const String;
    // convert the raw pointer to an integer
    let my_value_addr = my_value_raw_ptr as usize;
    println!("address = {:X}", my_value_addr);
}

#[test]
fn t4() {
    {
        let s1 = 10;
        let s2 = &s1;
        let s3 = s1;

        println!("{},{},{}", s1, s2, s3);
    }

    // {
    //     let s1 = String::from("abc");
    //     let s2 = &s1;
    //     let s3 = s1;
    //     // println!("{}", s1);
    //     println!("{},{}", s2, s3);
    // }

    {
        let s1 = Dog { age: 10 };
        let s2 = &s1;
        let s3 = s1;
        println!("{:?},{:?},{:?}", s1, s2, s3);
    }

    {
        //        let s1 = Pig { age: 10 };
        //        let s2 = &s1;
        //        let s3 = s1;
        // println!("{:?},{:?},{:?}", s1, s2, s3);
    }
}

#[derive(Copy, Clone)]
struct Dog {
    age: i32,
}

impl Debug for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.age).finish()
    }
}

struct Pig {
    age: i32,
}

impl Debug for Pig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.age).finish()
    }
}

// Move Copy Clone Drop

// 1. Copy trait, copy语义就是变量在stack内存的按位复制，没有其他任何多余的操, 高速，只能由编译器实现
// 2. 所有字段都实现Copy，可以 #[derive(Copy)] 宏来实现 Copy，，元组成员全部是copy，那么元组本身也copy
// 3. Clone约定了数据被深拷贝或浅拷贝的行为

// move copy 都是隐式的，，clone是显示的
// drop 和copy 不能同时实现

// 1. 如果没有实现Copy trait, 那么变量的赋值，传参，返回值，都是move操作，反之都是copy语义
// 2.
//

#[test]
fn t13() {
    let mut a = 10;
    let c = &mut a;
    // a = 11;
    // println!("{},{}", a, c);
}

#[test]
fn t14() {
    let ref j: i32;
    let i = 3;
    j = &i;
    println!("{}", j);

    let i = 30;
    //    let ref s = &i;
    let s = &i;
    println!("{}", s);

    let ref a = 2;
    let b = &2;
    println!("{}, {}", a, b);

    {
        let r = &1;
        let &a = r;
        let b = *r;
        println!("{}, {}", a, b);
    }
    {
        let ref x = &false; // x &&bool
        println!("x is &&bool: {}", x);
    }
}

#[test]
fn t15() {
    let ref j: i32;
    // j = 1;
    j = &1;
    println!("{}", j);
}
