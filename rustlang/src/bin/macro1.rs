use std::fmt;

#[test]
fn f1() {
    // assert!("a" == "b", "eq {}", 1);

    // assert_eq!("a", "b", "eq {}", 2);

    // assert_ne!("a", "a", "not eq")
}

#[test]
fn f2() {
    let cat = Cat { name: "z" };
    let cat2 = Cat { name: "z" };
    assert_eq!(cat, cat2, "?");
}

struct Cat {
    name: &'static str,
}

impl PartialEq for Cat {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Debug for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cat").field("name", &self.name).finish()
    }
}

fn main() {}
