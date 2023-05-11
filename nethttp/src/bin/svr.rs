use jsonrpsee::server::{RpcModule, ServerBuilder};
use std::net::SocketAddr;

async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = ServerBuilder::default().build("127.0.0.1:9900").await?;

    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| Ok("lo"))?;

    let addr = server.local_addr()?;
    let handle = server.start(module)?;

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    // tokio::spawn();
    handle.stopped().await;
    Ok(addr)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_server().await?;
    return Ok(());
}
