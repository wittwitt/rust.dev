use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, client::Subscription, Error};
use jsonrpsee::proc_macros::rpc;
// use jsonrpsee::server::{PendingSubscriptionSink,
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::{types::SubscriptionResult, SubscriptionSink};

use jsonrpsee::ws_client::WsClientBuilder;

type ExampleHash = [u8; 32];
type ExampleStorageKey = Vec<u8>;

///
/// namespace， method.name 区分大小写, eth_hi, eth_GetName

#[rpc(server, client, namespace = "eth")]
pub trait Rpc<Hash: Clone, StorageKey>
where
    Hash: std::fmt::Debug,
{
    #[method(name = "hi")]
    async fn hi(&self) -> Result<String, Error>;

    #[method(name = "GetName")]
    async fn get_name(&self) -> Result<String, Error>;

    #[method(name = "getKeys")]
    async fn storage_keys(
        &self,
        storage_key: StorageKey,
        hash: Option<Hash>,
    ) -> Result<Vec<StorageKey>, Error>;

    /// Subscription that takes a `StorageKey` as input and produces a `Vec<Hash>`.
    #[subscription(name = "subscribeStorage" , item = Vec<Hash>)]
    fn subscribe_storage(&self, keys: Option<Vec<StorageKey>>);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer<ExampleHash, ExampleStorageKey> for RpcServerImpl {
    async fn hi(&self) -> Result<String, Error> {
        Ok(String::from("this is hi"))
    }

    async fn get_name(&self) -> Result<String, Error> {
        Ok(String::from("this is get_name method"))
    }

    async fn storage_keys(
        &self,
        storage_key: ExampleStorageKey,
        _hash: Option<ExampleHash>,
    ) -> Result<Vec<ExampleStorageKey>, Error> {
        Ok(vec![storage_key])
    }

    fn subscribe_storage(
        &self,
        pending: SubscriptionSink,
        _keys: Option<Vec<ExampleStorageKey>>,
    ) -> SubscriptionResult {
        // let mut dsdp = pending.

        // pending.accept().unwrap();

        // let msg = SubscriptionMessage::from_json(&vec![[0; 32]])?;

        // pending.send(&"Response_A").unwrap();

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    let server_addr = run_server().await?;
    let url = format!("ws://{}", server_addr);

    let client = WsClientBuilder::default().build(&url).await?;
    assert_eq!(
        client
            .storage_keys(vec![1, 2, 3, 4], None::<ExampleHash>)
            .await
            .unwrap(),
        vec![vec![1, 2, 3, 4]]
    );

    let mut sub: Subscription<Vec<ExampleHash>> =
        RpcClient::<ExampleHash, ExampleStorageKey>::subscribe_storage(&client, None)
            .await
            .unwrap();
    assert_eq!(Some(vec![[0; 32]]), sub.next().await.transpose().unwrap());

    Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = ServerBuilder::default().build("127.0.0.1:9900").await?;

    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl.into_rpc())?;

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    // tokio::spawn(handle.stopped());

    handle.stopped().await;

    Ok(addr)
}
