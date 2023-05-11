use std::collections::{hash_map::RandomState, HashMap};

fn main() {}

#[test]
fn iter1_zip() {
    let mut v1 = HashMap::new();
    v1.insert("s1".to_string(), "v1_s1".to_string());
    v1.insert("s2".to_string(), "v2_s1".to_string());
    v1.insert("s3".to_string(), "v3_s1".to_string());
    let v11 = &v1;

    let mut v2 = HashMap::new();
    v2.insert("s1".to_string(), "v1_s2".to_string());
    v2.insert("s2".to_string(), "v2_s2".to_string());
    v2.insert("s3".to_string(), "v3_s2".to_string());

    let v22 = &v2;

    let v3: HashMap<&String, &String> = v11
        .iter()
        .zip(v22.into_iter()) // hash map    ，，， zip iter key的顺序不确定的
        .map(|(v1, v2)| (v1.1, v2.1))
        .collect(); //

    for (k, v) in v3 {
        println!("{},{}", k, v) // 结果混乱
    }

    println!("==================");

    let v3: HashMap<&String, String> = v11
        .iter()
        .map(|(v1k, v1v)| (v1v, v22[v1k].clone())) // 通过key 保证唯一
        .collect();
    for (k, v) in v3 {
        println!("{},{}", k, v) // 结果正确
    }
}

#[test]
fn iter1_next() {
    let s = RandomState::new();

    let mut v1 = HashMap::with_hasher(s);
    v1.insert("s1".to_string(), "v1_s1".to_string());
    v1.insert("s2".to_string(), "v2_s1".to_string());
    v1.insert("s3".to_string(), "v3_s1".to_string());

    // v1.iter().for_each(|(k, v)| println!("k: {}, v: {}", k, v));

    //惰性，只有next的时候，才一个一个执行
    let mut iter = v1.iter().map(|(k, v)| {
        println!("k: {}, v: {}", k, v);
        (k, v)
    });
    while let Some(item) = iter.next()
    //
    {
        println!("---");
        println!("{:?}", item);
    }

    //.for_each(|_| {});
}

trait Goo {
    fn say(&self) -> String;
}

struct S1 {}

impl Goo for S1 {
    fn say(&self) -> String {
        "s1".to_string()
    }
}

struct S2 {}

impl Goo for S2 {
    fn say(&self) -> String {
        "s2".to_string()
    }
}

#[test]
fn iter1_gen() {

    // let mut ls: Vec<T, T: Goo> = Vec::new();
    // let s1 = S1 {};
    // let s2 = S2 {};
    // ls.push(s1);
    // ls.push(s2);

    // ls.iter().for_each(|item| {
    //     item.say();
    // });
}
