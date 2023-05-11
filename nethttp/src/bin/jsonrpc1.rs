use std::{convert::Infallible, error::Error, net::SocketAddr};

use bytes::Bytes;

use http_body_util::{BodyExt, Full};

use hyper::server::conn::http1;
use hyper::{body::Bytes as HyperBytes, service::service_fn, Request, Response};

use tokio::net;

use jsonrpsee::{
    server::RpcModule,
    types::{request::Request as RpcReq, response::Response as RpcRes},
};

use serde_json as json;

async fn route(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<HyperBytes>>, Infallible> {
    println!("route");

    let mut module = RpcModule::new(());
    module
        .register_method("say_hello", |params, _| {
            // params.one::<u64>().map_err(Into::into)
            println!("params: {:?}", params);
            Ok(1)
        })
        .unwrap();

    match req.uri().path() {
        "/some" => {
            println!("/some");
            Ok(Response::new(Full::new(HyperBytes::from("some World!"))))
        }
        "/rpc" => {
            let (parts, body) = req.into_parts();
            let bd = body.collect().await.unwrap().to_bytes();

            let jsonrpc_req: RpcReq = json::from_slice(&bd).unwrap();

            let method = jsonrpc_req.method.trim();

            println!(
                "url: {},id: {}, method: {}",
                parts.uri,
                jsonrpc_req.id.as_str().unwrap(),
                method
            );

            let params2 = jsonrpc_req.params.unwrap();

            let mut p3 = jsonrpsee::core::params::ArrayParams::new();
            p3.insert(params2).unwrap();

            let res: json::Value = module.call(method, p3).await.unwrap();

            let resff = RpcRes::new(res, jsonrpc_req.id);

            Ok(Response::new(Full::new(Bytes::from(resff.to_string()))))
        }
        _ => Ok(Response::new(Full::new(Bytes::from("404")))),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9900));

    let listener = net::TcpListener::bind(addr).await?;

    println!("{:?}", listener.local_addr().unwrap().to_string());

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(route))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
