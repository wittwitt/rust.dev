fn main() {}

#[test]
fn t1() {
    let mut s = String::from("abcd");
    let f = || {
        println!("f: {}", s);
    };
    some(f);

    let f2 = || {
        s.push_str("efgh");
        println!("f2: {}", s)
    };
    some2(f2);
    println!("{}", s);
}

#[test]
fn t2() {
    let s = String::from("abcd");
    let f = || {
        println!("f: {}", s);
    };
    some(f);
    some2(f);
    some3(f);

    let f2 = || println!("f2: abcd");
    some(f2);

    println!("last {}", s);
}

pub fn some<T>(f: T)
where
    T: Fn(),
{
    f();
}

pub fn some2<T>(mut f: T)
where
    T: FnMut(),
{
    f();
}

pub fn some3<T>(f: T)
where
    T: FnOnce(),
{
    f();
}

/*

1. 注意匿名函数和闭包区别，，

闭包（closure），是一种匿名函数，并且具有“捕获”外部变量的能力

2.  closure 会创建一个匿名的struct，编译器会在当前上下文捕获 closure 代码中的外部变量然后塞进这个结构体里面

3. Rust 中结构体的可变性以及 liftime 本身就很烦人。

4. Closure 的规则都是隐式的：closure 捕获值的方式及所生成的closure的类型都是按照隐式的规则决定的。

5. Closure 一直会捕获整个复合类型，如 struct, tuple 和 enum 。而不只是单个字段[2]。

6. Rust目前的closure实现，又叫做unboxed closure


关于第一个问题，Rust主要是通过分析外部变量在闭包中的使用方式，通过一系列的规则自动推导出来的。主要规则如下：

    如果一个外部变量在闭包中，只通过借用指针&使用，那么这个变量就可通过引用&的方式捕获；
    如果一个外部变量在闭包中，通过& mut指针使用过，那么这个变量就需要使用&mut的方式捕获；
    如果一个外部变量中闭包中，通过move的方式使用过，那么这个变量就需要使用“by value”的方式捕获。

*/

#[test]
fn t3() {
    let mut i = 3;

    let f = move || i + 1;

    i = 4;
    println!("{}", i);

    println!("{}", f());
    println!("{}", f());

    i = 4;

    println!("{}", i);
}

#[test]
fn t4() {
    let i = 3;
    let f = || i + 1;
    println!("{}", f());
}

#[test]
fn t5() {
    let i = 3;
    let f = || i + 1;
    ff2(f);
}

struct Cat {
    age: i32,
}

#[test]
fn t6() {
    {
        let i = 3;
        let f = || i + 1;
        ff1(f);
        ff1(&f);
    }

    {
        let i = Cat { age: 10 };
        let f = || {
            println!("xxx{}", i.age);
            i.age + 1
        };
        ff3(f);
        ff3(&f);
    }
}

//
pub fn ff1<T: Fn() -> i32>(f: T) {
    println!("ff1 1:{:?}", f());
    println!("ff1 2:{:?}", f());
}

pub fn ff2<T: FnMut() -> i32>(mut f: T) {
    println!("ff2 1:{:?}", f());
    println!("ff2 2:{:?}", f());
}

//
pub fn ff3<T>(f: T)
where
    T: FnOnce() -> i32,
{
    println!("ff3 1:{:?}", f());
    // println!("f2:{:?}", f());
}

#[test]
fn t7() {
    fn f<F: FnOnce() -> String>(g: F) {
        println!("{}", g());
    }

    let mut s = String::from("foo");
    let t = String::from("bar");

    f(|| {
        s += &t;
        s
    });

    // f(|| {
    //     s += &t;
    //     s
    // });
}

#[test]
fn t8() {
    fn f<F: FnOnce() -> i32>(g: F) {
        println!("{}", g());
    }

    let mut s = 3;
    let t = 4;

    let s1 = String::from("abc");

    let ff = || {
        let s2 = s1.clone() + "";
        // s += &t;
        s
    };

    f(&ff);
    f(&ff);

    // let ff2 = || s + 1;
    // f(&ff2);
    // f(&ff2);
}

// 1. closure 实现那个trait，看外面变量在内部的使用方式

// Fn FnMut FnOnce

// 作为变量
// 作为参数
// 作为返回值

#[test]
fn t9() {
    let mut name = String::from("Ethan");

    let mut i = 10;

    let print_info_closure = |age| {
        // Fn

        // i = i + 1; // FnMut

        println!("name is {}", name);
        println!("age is {}", age);
    };

    // println!("{}", name);
    // name = String::from("abc");

    // print_info_closure(100);

    t9some(100, print_info_closure);
    t9some(100, print_info_closure);
}

pub fn t9some<F: Fn(i32)>(age: i32, f: F) {
    f(age);
}

#[test]
fn t10() {
    // println!("{}", t10som2(&t10some));
    println!("{}", t10som2(t10some));

    let f = |i: i32| i + 1;
    println!("{}", t10som2(f));
    // println!("{}", t10som2(&f));

    println!("{}", t10som3(f));
    println!("{}", t10som3(&f));
}

pub fn t10some(i: i32) -> i32 {
    return i + 1;
}

pub fn t10som2(f: fn(i32) -> i32) -> i32 {
    return f(3);
}

pub fn t10som3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(3);
}

// 引用，指针，借用

/*
1. 闭包可以捕获上下文变量，函数不可以
*/

#[test]
fn t12() {
    {
        let mut a = [1, 2, 3];
        let x = &mut a;

        (*x)[1] = 100;

        let c = &x; // x 是不可变， &x，是对不可以边的引用
        let d = &c; // 可以多次引用
        println!("{:?}", d);
    }

    let mut a = [1, 2, 3];
    let x = &mut a;
    {
        let mut c = || {
            (*x)[0] = 0; // 修改一个可变引用的引用时， 造成x这里只能是唯一不可以变引用，，，不能再&x，，
        };
        // let y = &x; // 报错 second borrow occurs here
        c();
    }
}

// move 强制所有权转移到closure，但不影响trait
