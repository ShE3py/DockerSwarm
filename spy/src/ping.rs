//!
//! Uility for pinging a websocket.
//!

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use tungstenite::{Bytes, Message};

/// Returns `Ok` if the worker at address `ip:3000` is responding,
/// `Err` otherwise.
pub fn ping(ip: IpAddr) -> Result<(), ()> {
    let addr = SocketAddr::new(ip, 3000);
    let stream = match TcpStream::connect(addr) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("connect: {e}");
            return Err(());
        }
    };
    if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
        eprintln!("setsockopt: {e}");
        return Err(());
    }
    
    let (mut socket, response) = match tungstenite::client("ws://localhost:3000/socket", stream) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("handshake: {e}");
            return Err(());
        }
    };
    
    if response.status() != 101 /* Switching Protocol */ {
        eprintln!("status: {}", response.status());
        return Err(());
    }
    
    if let Err(e) = socket.send(Message::Ping(Bytes::from_static(b"xoxo"))) {
        eprintln!("send: {e}");
        return Err(());
    }
    
    match socket.read() {
        Ok(_) => {
            // we don't care about the message we received, the worker's up so all's good
            println!("OK");
        }
        Err(e) => {
            eprintln!("recv: {e}");
            return Err(());
        }
    }
    
    _ = socket.close(None);
    Ok(())
}
