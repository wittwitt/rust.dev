fn main() {
    say!(12);

    let s = "abc";
    say!(s);

    say!(1, 2, 3, 4, 5,);
}

#[macro_export]
macro_rules! say {
    ($x:ident) => {
        println!("haha: {:?}", $x);
    };

    ($x:expr) => {
        println!("ssss: {:?}", $x);
    };

    ( $($args:expr,)* ) => {
        for item in [$($args),*] {
            println!("xxxx: {:?}", item);
        }
    };
}
