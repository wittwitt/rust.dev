fn main() {
    let c = [1, 2, 3]; //
    println!("{}", c[2]);

    let c2 = [8; 10]; // [初始化，总量]
    println!("c2:{:?}", c2);

    {
        let c3 = [1, 2, 3, 4, 5, 6];
        let ss: &[i32] = &c3[2..4]; // slice
        println!("c3:{:?}", ss)
    }
}
