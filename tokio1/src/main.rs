use tokio::runtime;

fn main() {
    println!("Hello, world!");

    let rt = runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut c = Vec::new();

        for _i in 0..10 {
            let jh = rt.spawn(say_hi());
            c.push(jh);
        }

        for jh in c {
            jh.await.unwrap();
        }
    })
}

async fn say_hi() {
    println!("hi");
}
