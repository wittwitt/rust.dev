use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(5));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(loo());

    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn loo() {
    let mut list = Vec::new();

    for i in 0..2000 {
        list.push(i);
    }

    for i in 0..2000 {
        let map_op = i;
        tokio::spawn(async move {
            println!("{}", map_op);

            // std::thread::sleep(std::time::Duration::from_secs(1));
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        });
    }

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
