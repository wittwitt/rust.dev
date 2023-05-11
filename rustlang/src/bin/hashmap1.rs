use std::collections::HashMap;

fn main() {}

#[test]
fn t1() {
    let mut m = HashMap::new();
    m.insert("a", "a1");
    m.insert("b", "b1");
    m.iter().for_each(|(k, v)| println!("k: {}, v: {}", k, v));

    m.entry("a").or_insert("a2"); // if not exist, insert, or not update
    m.entry("c").or_insert("c1");

    m.iter().for_each(|(k, v)| println!("k: {}, v: {}", k, v));
}

#[test]
fn hashmap_1() {
    let mut v1 = HashMap::new();
    v1.insert("s1".to_string(), "v1_s1".to_string());
    v1.insert("s2".to_string(), "v2_s1".to_string());
    v1.insert("s3".to_string(), "v3_s1".to_string());

    v1.iter().for_each(|(k, v)| println!("{},,{}", k, v))
}

#[test]
fn hashmap_2() {
    let mut v1: HashMap<String, String> = HashMap::new();
    v1.insert("s1".to_string(), "v1_s1".to_string());
    v1.insert("s2".to_string(), "v2_s1".to_string());
    v1.insert("s3".to_string(), "v3_s1".to_string());

    v1.iter().for_each(|(k, v)| println!("{},,{}", k, v))
}

#[test]
fn hashmap_merge() {
    let mut v1: HashMap<String, String> = HashMap::new();
    v1.insert("s1".to_string(), "v1_s1".to_string());
    v1.insert("s2".to_string(), "v2_s1".to_string());
    v1.insert("s3".to_string(), "v3_s1".to_string());

    let mut v2: HashMap<String, String> = HashMap::new();
    v1.insert("s21".to_string(), "v1_s21".to_string());
    v1.insert("s22".to_string(), "v2_s21".to_string());
    v1.insert("s23".to_string(), "v3_s21".to_string());
}
