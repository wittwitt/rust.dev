#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E {}", self.a);
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(),
{
    println!("fn_once begins");
    func();
    // func();
    println!("fn_once ended");
}

fn main() {
    let e = E {
        a: "fn_once".to_string(),
    };
    // 这样加个move，看看程序执行输出顺序有什么不同
    // let f = move || println!("fn once calls: {:?}", e);
    let f = || println!("fn once closure calls: {:?}", e);

    fn_once(f);
    println!("main ended");
}

pub fn is_fn<F: Fn() -> String>(f: F) {
    f();
}

#[test]
fn t1() {
    let s = String::from("hello");
    let closure = || s.clone() + "abc";
    is_fn(closure);
}

pub fn some() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|i: i32| i + 1)
}

#[test]
fn t2() {
    let s = some();
    let k = s(1);
    println!("{}", k);
}

#[test]
fn t4() {
    // let names: Vec<Vec<String>> = vec![
    //     vec!["Foo1".to_string(), "Foo2".to_string()],
    //     vec!["Bar1".to_string(), "Bar2".to_string()],
    // ];
    // let ids: Vec<i64> = vec![10, 20];

    // names
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(i, v)| v.iter().map(|n| (n.clone(), ids[i])));
}

#[test]
fn t5() {
    // let mut i = 0;
    // let closure = || {
    //     i = 2;
    //     || {
    //         println!("i = {}", i);
    //     }
    // };
    // closure()();
}

#[test]
fn t52() {
    let mut i = 0;
    let mut closure = || {
        i = 3;
        let mut j = i + 1;
        println!("j = {}", j);
        move || {
            j = j + 1;
            println!("j2 = {}", j);
            println!("i = {}", i);
        }
    };
    let mut s = closure();
    s();
}

#[test]
fn t6() {
    let mut i = 3;

    let mut c1 = || {
        i = 2;
        3
    };
    c1();

    // why mut closure

    /*
    struct S<'a>(&'a mut String);
    impl S<'_> {
        fn call(&mut self) {
            self.0.push_str("yo");
        }
    }
    fn main() {
        let mut s = "hi".to_string();
        let c = S(&mut s); // c mut , c.call change S
        c.call();
    }
    */
}

pub struct T7st {
    age: i32,
}

impl T7st {
    fn say(&self) {
        println!("I am t7st");
    }

    fn say2(&mut self) {
        self.age = self.age + 1;
    }
}

#[test]
fn t7() {
    let mut s1 = T7st { age: 3 };
    s1.say();
    s1.say2();
}

#[test]
fn t8() {
    let i = 3;
    let c1 = || {
        let j = i + 1;
        j
    };
    c1();
}

