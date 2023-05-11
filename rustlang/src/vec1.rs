mod t1 {
    #[test]
    fn t1() {
        let mut v1 = Vec::new();

        v1.push(1);
        v1.push(2);
        v1.push(3);
        v1.push(4);

        for item in v1.iter() {
            println!("{}", item);
        }

        println!("----");

        v1.pop();
        v1.pop();

        for item in v1.iter() {
            println!("{}", item);
        }
    }
}
