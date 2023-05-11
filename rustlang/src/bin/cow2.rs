use std::borrow::Cow;

fn main() {}

#[test]
fn t1() {
    let cat = Cat::new("abc");
    println!("{}", &cat.name);

    let cat2 = Cat::new2("abc2");
    println!("{}", &cat2.name);
}

struct Cat {
    name: Cow<'static, str>,
}

impl Cat {
    fn new(str: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(str),
        }
    }

    fn new2(str: impl Into<Cow<'static, str>>) -> Self {
        Self { name: str.into() }
    }
}
