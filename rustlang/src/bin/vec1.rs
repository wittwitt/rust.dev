use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
// use std::str::FromStr;

// use rayon::iter::Inspect;
use rayon::prelude::{
    // IndexedParallelIterator,
    IntoParallelIterator,
    // IntoParallelRefIterator,
    ParallelIterator,
};

fn main() {
    // v2();
    // v3();
    // v4();
    // v6();
    // v7();
    v8();
}

#[allow(dead_code)]
fn v1() {
    let v = [91i32, 2, 3, 10, 4];

    let sum = v.iter().fold(10, |a, &b| a + b);
    println!("{:?}", sum);

    let sum = v.into_iter().fold(10, |a, b| a + b);
    println!("{:?}", sum);

    // let sum = v.iter().reduce(|a, b| {
    //     let c = a + b;
    //     c
    // });
    // println!("{:?}", sum);

    let sum = v.into_iter().reduce(|a, b| a + b);
    println!("{:?}", sum);
}

#[allow(dead_code)]
fn v2() {
    let mut a = BTreeSet::new();
    a.insert(3);
    a.insert(4);
    a.insert(100);
    a.insert(2);

    let mut b = BTreeSet::new();
    b.insert(3);
    b.insert(4);
    b.insert(5);

    a.append(&mut b);

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);

    for ii in a.into_iter() {
        println!("{:?}", ii);
    }

    {
        let mut a = BTreeSet::new();
        a.insert("acb");
        a.insert("acd");
        a.insert("abc");

        for ii in a.into_iter() {
            println!("{:?}", ii);
        }
    }
}

#[allow(dead_code)]
fn v3() {
    let v = Vec::from([1, 2, 3, 4, 5]);

    for (i, item) in v.chunks(2).enumerate() {
        println!("{:?},{:?}", i, item)
    }
}

#[allow(dead_code)]
fn v4() {
    let n: usize = 10;
    let (inclusion_proofs, faults) = (0..n)
        .into_par_iter()
        .fold(
            || (Vec::new(), BTreeSet::new()),
            |(mut inclusion_proofs, mut faults), n| {
                //
                //
                println!("xxxx{:?}", n);

                inclusion_proofs.push(n);
                faults.insert(n);

                println!("xxxx====={:?}", inclusion_proofs);
                println!("xxxx=====33333{:?}", faults);

                //
                //
                (inclusion_proofs, faults)
            },
        )
        .reduce(
            || (Vec::new(), BTreeSet::new()),
            |(mut inclusion_proofs, mut faults), (p, f)| {
                println!("==={:?}", p);
                inclusion_proofs.extend(p);
                faults.extend(f);
                (inclusion_proofs, faults)
            },
        );

    println!("{:?},{:?}", inclusion_proofs, faults);
}

#[allow(dead_code)]
fn v5() {
    let mut leaf_map: HashMap<String, (Vec<(u64, u64)>, Option<String>)> = HashMap::new();

    leaf_map.insert(
        String::from("/path/1"),
        (Vec::new(), Some(String::from("some"))),
    );

    leaf_map.insert(
        String::from("/path/2"),
        (Vec::new(), Some(String::from("some"))),
    );

    leaf_map.insert(
        String::from("/path/3"),
        (Vec::new(), Some(String::from("some"))),
    );
}

#[allow(dead_code)]
fn v6() {
    let mut bt = BTreeSet::new();

    bt.insert(1);

    // bt.clear();

    println!("{},{}", bt.len(), bt.is_empty());
}

#[allow(dead_code)]
fn v7() {
    let mut map = BTreeMap::new();
    map.insert(3, "c");
    map.insert(2, "b");
    map.insert(1, "a");

    for item in map.iter() {
        println!("{},{}", item.0, item.1)
    }

    let c = Some(3);

    let x = match c {
        Some(4) => 4,
        _ => {
            return;
        }
    };

    println!("{:?}", x);
}

#[allow(dead_code)]
fn v8() {
    let mut vec = Vec::with_capacity(10);
    for item in 0..10 {
        vec.push(item)
    }

    vec.push(10000);
    assert_eq!(20, vec.capacity()); // 容量翻倍
}

#[test]
fn vec1_slice() {
    let ls = vec![5, 6, 2, 7, 10];
    let ls2 = &ls[2..4]; // 截取一部分
    ls2.iter().for_each(|item| println!("{}", item));
}

#[test]
fn vec1_append() {
    let mut ls = vec!["s1".to_string(), "s2".to_string(), "s3".to_string()];
    let mut ls2 = vec!["s11".to_string(), "s22".to_string(), "s33".to_string()];

    ls.append(&mut ls2);

    ls.iter().for_each(|item| println!("{}", item));

    println!("{}", ls2.len()); // len = 0
}

struct Cat {
    age: i32,
    name: &'static str,
}

#[test]
fn vec1_append2() {
    let mut ls = vec![
        Cat { age: 1, name: "1" },
        Cat { age: 2, name: "2" },
        Cat { age: 3, name: "3" },
    ];
    let mut ls2 = vec![
        Cat {
            age: 11,
            name: "11",
        },
        Cat {
            age: 22,
            name: "22",
        },
        Cat {
            age: 33,
            name: "33",
        },
    ];

    ls.append(&mut ls2);

    ls.iter().for_each(|item| println!("{}", item.name));

    println!("{}", ls2.len()); // len = 0
}
