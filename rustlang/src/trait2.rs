use log::info;

use crate::MyTrait3;

pub trait Say {
    fn say(&self) {
        info!("{}", "some");
        println!("hi");
    }
}

impl Say for MyTrait3 {
    fn say(self: &Self) {
        println!("t2 say hi");
    }
}
