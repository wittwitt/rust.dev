// mod super::m11; // 不需要mod，mod只能用在mod.rs里引用
use super::m11::Cat; // super = parent mod

pub fn say() {
    let cat = Cat {};
    cat.say();
    cat.say2();
}

impl Cat {
    fn say(&self) {
        println!("cat m22 say")
    }
}
