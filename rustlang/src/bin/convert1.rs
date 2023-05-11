fn main() {
    let i: i32 = 10;
    let j = i.to_string();
    println!("{}", j);

    let k = j.parse::<i32>().unwrap();

    println!("{}", k + 1);
}
