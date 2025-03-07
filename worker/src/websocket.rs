//!
//! WebSocket server utility.
//!

#![expect(dead_code, reason = "either `read` or `broadcast` is unused")]

use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process::exit;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::{Message, Utf8Bytes};

/// A client socket.
pub(super) type Leech = ::tungstenite::WebSocket<TcpStream>;

/// A WebSocket server socket.
pub(super) struct WebSocket {
    /// The socket.
    socket: TcpListener,
    
    /// The connected client sockets.
    clients: Vec<Leech>,
}

impl WebSocket {
    /// Bind a new WebSocket server to a port.
    pub(super) fn new(port: u16) -> WebSocket {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
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
        
        println!("Server started on ws://{}", server.local_addr().unwrap_or(addr));
        WebSocket {
            socket: server,
            clients: Vec::new(),
        }
    }
    
    /// Handle incomming client sockets (i.e. handshaking).
    pub(super) fn accept(&mut self) {
        loop {
            match self.socket.accept() {
                Ok((stream, _)) => match tungstenite::accept(stream) {
                    Ok(ws) => {
                        match ws.get_ref().set_nonblocking(true) {
                            Ok(()) => self.clients.push(ws),
                            Err(e) => eprintln!("ws ioctl(): {e}"),
                        };
                    },
                    Err(e) => {
                        eprintln!("ws handshake: {e}");
                    }
                },
                Err(e) if e.kind() != ErrorKind::WouldBlock => {
                    eprintln!("accept(): {e}");
                },
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break;
                },
                _ => unreachable!(),
            }
        }
    }
    
    /// Remove closed client sockets.
    pub(super) fn cleanse(&mut self) {
        self.clients.retain(Leech::can_write);
    }
    
    /// Read from all clients. `f` is called with the client socket, and the message it sent.
    pub(super) fn read<F: FnMut(&mut Leech, Message)>(&mut self, mut f: F) {
        for leech in &mut self.clients {
            let msg = match leech.read() {
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
            
            f(leech, msg);
        }
    }
    
    /// Send a message to a client socket. Terminate the client on error.
    pub(super) fn send(leech: &mut Leech, message: impl Into<Utf8Bytes>) {
        if let Err(e) = leech.send(Message::text(message)) {
            eprintln!("ws send: {e}");
            
            _ = leech.close(Some(CloseFrame {
                code: CloseCode::Error,
                reason: "bye bye!".into(),
            }));
        }
    }
    
    /// Broadcast a message to all client sockets.
    pub(super) fn broadcast(&mut self, message: impl Into<Utf8Bytes>) {
        let message = message.into();
        for leech in &mut self.clients {
            Self::send(leech, &*message);
        }
    }
}
