use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;

#[rpc]
pub trait Rpc {
    #[rpc(name = "pool.add")]
    fn add(&self, a: u64, b: u64) -> Result<u64>;

    #[rpc(name = "hi")]
    fn hi(&self, name: String) -> Result<bool>;

    #[rpc(name = "err2")]
    fn err2(&self) -> Result<String>;
}

pub struct Pool;
impl Rpc for Pool {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }

    fn hi(&self, name: String) -> Result<bool> {
        println!("{}", name);
        Ok(true)
    }

    fn err2(&self) -> Result<String> {
        Err(jsonrpc_core::error::Error {
            code: jsonrpc_core::ErrorCode::ServerError(300),
            message: "haha".to_string(),
            data: None,
        })
    }
}

// fn main() {
//     let mut io = jsonrpc_core::IoHandler::new();
//     io.extend_with(RpcImpl.to_delegate())
// }

fn main() {
    // let mut io = IoHandler::default();
    // io.add_method("say_hello", |_params: Params| async {
    //     Ok(Value::String("hello".to_owned()))
    // });

    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(Pool.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:9900".parse().unwrap())
        .unwrap();

    server.wait();
}
