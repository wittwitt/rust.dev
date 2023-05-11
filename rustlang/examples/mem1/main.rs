fn main() {
    loop {
        f1();
        println!("=====");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn f1() {
    let mut cc = Vec::new();
    for _j in 0..1024 * 1024 * 1024 {
        cc.push("a");
    }
    println!("111111");

    let mut v = Vec::new();
    v.push(vec!["a"]);
    std::thread::sleep(std::time::Duration::from_secs(5));
    for i in 0..100 {
        println!("xxxx: {}", i);
        v.push(cc.clone());
    }
}
