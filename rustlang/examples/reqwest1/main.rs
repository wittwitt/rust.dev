use std::{str, time};

use reqwest::{
    header::{
        // HeaderValue,
        // CONTENT_RANGE,
        CONTENT_TYPE,
        RANGE,
    },
    Client as HTTPClient,
    //  Method,
    RequestBuilder as HTTPRequestBuilder,
    //  Response as HTTPResponse,
    StatusCode,
    // Url,
};

#[tokio::main]
async fn main() {
    flexi_logger::Logger::try_with_str("trace")
        .unwrap()
        .start()
        .unwrap();

    f2().await;
}

#[allow(unused)]
fn f1() {
    let res = reqwest::blocking::get("https://www.baidu.com");

    match res {
        Ok(res) => {
            let r = res.bytes().unwrap();
            let body = String::from(str::from_utf8(&r).unwrap());
            println!("{:?}", body);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

async fn f2() {
    let url = "http://127.0.0.1:8080/p1";

    let connect_timeout = time::Duration::from_secs(60);
    let send_timeout = time::Duration::from_secs(13);

    let client = HTTPClient::builder()
        .connect_timeout(connect_timeout)
        .timeout(send_timeout)
        .connection_verbose(true) // true
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(connect_timeout)
        .no_proxy() // http_proxy https_proxy 影响了timeout close问题
        .http2_max_frame_size(5)
        .build()
        .expect("failed to build Reqwest Client");

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(r) => {
                println!("ok {:?}", r);
            }
            Err(e) => {
                println!("eee {:?}", e);
            }
        }
    }
}
