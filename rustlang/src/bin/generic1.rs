fn main() {
    {
        let d = Dinosaur { x: "abc" };
        println!("{}", d.x);
    }
    {
        let d = Dinosaur {
            x: "abc".to_string(),
        };
        println!("{}", d.x);
    }
}

struct Dinosaur<T> {
    x: T,
}
