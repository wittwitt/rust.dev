use rustlang::trait1::Say as t1say;
use rustlang::trait2::Say as othesay;
use rustlang::MyTrait3;

fn main() {
    let mut t3 = MyTrait3 { name: "xxx" };

    let mut t4 = &mut t3;

    t4.name = "abc";

    println!("{}", t3.name);

    let t3p = &t3;

    // t3p.say();
    t1say::say(t3p);

    // t3.say();

    let t3p2 = &t3;

    <MyTrait3 as t1say>::say(t3p2);

    t1say::say(t3p2);

    // t3p.say();

    othesay::say(&t3);

    // let mut t5 = &mut t3;
    // t5.name = "abc";
    // t5.say();

    // t4.name = "ccc";
}
