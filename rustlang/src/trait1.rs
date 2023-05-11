use crate::MyTrait3;

pub trait Say {
    fn say(self: &Self) {
        println!("say: ");
    }
}

impl Say for MyTrait3 {
    fn say(&self) {
        println!("t1 say: ");
    }
}
