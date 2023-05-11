use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
    string::String,
};

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start_server());
}

async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:19900").unwrap();

    loop {
        let (socket, _) = listener.accept().unwrap();
        tokio::spawn(async move {
            let mut ss = socket;
            process(&mut ss).await;
        });
    }
}

async fn process(socket: &mut TcpStream) {
    loop {
        let mut buf = [0; 1024];

        let n = match socket.read(&mut buf) {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        if n > 0 {
            let c = str::from_utf8(&buf).unwrap();
            let cstr = String::from(c);
            println!("c->s: {}", cstr);
        }

        if let Err(e) = socket.write_all(&buf[0..n]) {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}
