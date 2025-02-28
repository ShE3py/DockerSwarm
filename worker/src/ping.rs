use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::process::exit;
use std::time::Duration;
use tungstenite::{Bytes, Message};

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3000);
    let stream = match TcpStream::connect(addr) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("connect: {e}");
            exit(1);
        }
    };
    if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
        eprintln!("setsockopt: {e}");
        exit(1);
    }
    
    let (mut socket, response) = match tungstenite::client("ws://localhost:3000/socket", stream) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("handshake: {e}");
            exit(1);
        }
    };
    
    println!("Connected to the server, version {:?}", response.version());
    println!("Status: {}", response.status());
    println!("Headers:");
    for (header, value) in response.headers() {
        println!("* {header} = {value}", value = String::from_utf8_lossy(value.as_bytes()));
    }
    
    if let Err(e) = socket.send(Message::Ping(Bytes::from_static(b"xoxo"))) {
        eprintln!("send: {e}");
        exit(1);
    }
    
    match socket.read() {
        Ok(m) => {
            println!("{m:?}");
        }
        Err(e) => {
            eprintln!("recv: {e}");
            exit(1);
        }
    }
    
    _ = socket.close(None);
}
