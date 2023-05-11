use tokio::runtime::Runtime;

async fn async_read(str: &str) -> String {
    println!("{}", str);
    String::from("abc")
}

async fn hello() {
    let content = async_read("a.txt").await;
    println!("{}", content);
    let content = async_read("b.txt").await;
    println!("{}", content);
}

fn main() {
    let future = hello();
    let rt = Runtime::new().unwrap();
    rt.block_on(future);

    let rt = tokio::runtime::Runtime::new().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
}
