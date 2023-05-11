// 1, 内联
//

const S: i32 = 10;

fn main() {
    println!("{}", S);
}

pub mod t1 {
    const C: Cat = Cat { age: 11 };

    struct Cat {
        age: i32,
    }

    impl Drop for Cat {
        fn drop(&mut self) {
            println!("drop")
        }
    }

    pub fn f1() {
        println!("start");
        println!("{}", C.age);
        println!("end");
        println!("{}", C.age);
    }

    #[test]
    fn t1() {
        f1();
    }
}
