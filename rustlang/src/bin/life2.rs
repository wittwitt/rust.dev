fn main(){

}


#[allow(dead_code)]
fn te() {
    let c = Cat {};
    println!("{}", c.hi());
    println!("{}", c.hi2());
    println!("{}", c.hi3());
}

// hi真正的编译和hi2一样,,,,hi3不同

// str 没有lifetime
// fn say() -> &str {
//     "this is str"
// }

#[allow(dead_code)]
fn say(k: &str) -> &str {
    let c = format!("say: {}", k);

    println!("{}", c);

    "c.as_str()"
}

#[allow(dead_code)]
fn say2() -> String {
    "c.as_str()".to_string()
}

#[allow(dead_code)]
fn say3(v: &mut String) -> &String {
    let c = "abc".to_string();

    v.push_str(c.as_str());

    v
}

// in a nutshell, this is why your code is illegal
// #[allow(dead_code)]
// fn say4<'a>(v: &'a mut String) -> &'a String {
//     let c = "abc".to_string();
//     v.push_str(c.as_str());
//     let k: &'a String = &String::from(v.as_str());
//     k
// }

trait Animal {
    fn say(&self) -> &str {
        "deafult say"
    }
}

struct Cat {}

impl Cat {
    fn hi(&self) -> &str {
        "hi"
    }

    fn hi2<'a>(&'a self) -> &'a str {
        "hi2"
    }

    fn hi3<'a>(&'a self) -> &'static str {
        "hi3"
    }
}
