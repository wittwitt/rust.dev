use std::collections::BTreeMap;

fn main() {}

#[test]
fn f1() {
    let mut b = BTreeMap::new();

    b.insert("111", "cccc");
    b.insert("a", "a1");
    b.insert("b", "b1");
    b.insert("a", "a2");
    b.insert("a", "a3");
    b.insert("d", "d1");
    b.insert("c", "c1");

    b.iter().for_each(|(k, v)| {
        println!("{},{}", k, v);
    });
}
