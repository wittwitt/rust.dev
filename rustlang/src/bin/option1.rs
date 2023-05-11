fn main() {}

#[test]
fn t1() {
    let mut list = Some(vec![]);

    // option as_ref
    if let Some(list) = &mut list {
        list.push("abc");
    }
}
