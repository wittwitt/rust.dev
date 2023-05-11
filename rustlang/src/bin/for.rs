struct Pool<'a> {
    list: &'a mut Vec<i32>,
}

fn main() {
    for i in 0..2 {
        let mut priv_sectors = Vec::with_capacity(33);
        priv_sectors.push(1);

        let p = &mut Pool {
            list: &mut priv_sectors,
        };

        p.list.push(1);

        sa(p);
        println!("{}", i)
    }
}

fn sa(p: &Pool) {}
