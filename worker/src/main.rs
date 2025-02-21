use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener};
use std::process::exit;
use std::str::FromStr as _;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::{Message, WebSocket};

fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:8000").unwrap();

    let server = match TcpListener::bind(addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("bind(): {e}");
            exit(1);
        },
    };

    if let Err(e) = server.set_nonblocking(true) {
        eprintln!("ioctl(): {e}");
        exit(1);
    };

    let mut clients = Vec::new();

    println!("Server started on {addr}");
    loop {
        // Accept incoming connections.
        match server.accept() {
            Ok((stream, _)) => match tungstenite::accept(stream) {
                Ok(ws) => {
                    match ws.get_ref().set_nonblocking(true) {
                        Ok(()) => clients.push(ws),
                        Err(e) => eprintln!("ws ioctl(): {e}"),
                    };
                },
                Err(e) => {
                    eprintln!("ws handshake: {e}");
                    continue;
                }
            },
            Err(e) if e.kind() != ErrorKind::WouldBlock => {
                eprintln!("accept(): {e}");
                continue;
            },
            _ => {},
        };

        // Read from all clients.
        for ws in &mut clients {
            let msg = match ws.read() {
                Ok(m) => m,
                Err(e) => {
                    match e {
                        tungstenite::Error::Io(e) if e.kind() == ErrorKind::WouldBlock => {},
                        tungstenite::Error::AlreadyClosed => unreachable!("ws read: {e}"),
                        _ => eprintln!("ws read: {e}"),
                    }

                    continue;
                },
            };

            match msg {
                Message::Text(s) => {
                    let s = s.trim();
                    let result = worker::md5break(s).unwrap_or_else(|e| format!("{e}"));
                    println!("Broke {s:?} for {:?}: {result:?}", ws.get_ref().peer_addr().map_or_else(|e| format!("<error: {e}>"), |addr| addr.to_string()));

                    if let Err(e) = ws.send(Message::text(result)) {
                        eprintln!("ws send: {e}");

                        _ = ws.close(Some(CloseFrame {
                            code: CloseCode::Error,
                            reason: "could not send a message".into(),
                        }));
                    }
                }
                Message::Binary(_) => {}
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Close(_) => {}
                Message::Frame(_) => {}
            }
        }

        // Remove closed connections.
        clients.retain(WebSocket::can_write);
    }
}
