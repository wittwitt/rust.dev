use std::fmt::Debug;

fn main() {}

pub enum Stat {
    No,
    S1,
    S2(String),
}

#[test]
fn t1() {
    let o1 = Option::<String>::None;
    let o2 = Option::Some("ok");

    println!("{:?},{:?}", o1, o2)
}

pub enum Sa<T> {
    No,
    S1(T),
    S2(String),
}

impl<T> Debug for Sa<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()",)
    }
}

#[test]
fn t2() {
    let o1 = Sa::<i32>::No;
    println!("{:?}", o1);
}
