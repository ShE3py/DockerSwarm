use crate::websocket::WebSocket;
use std::process::exit;
use std::thread;
use tungstenite::Message;

mod websocket;

fn main() {
    ::ctrlc::set_handler(|| {
        exit(130);
    }).unwrap();
    
    let mut server = WebSocket::new(3000);
    
    loop {
        server.cleanse();
        server.accept();
        server.read(|leech, msg| {
            if let Message::Text(s) = msg {
                let s = s.trim();
                let result = worker::md5break(s).unwrap_or_else(|e| format!("{e}"));
                println!("Broke {s:?} for {:?}: {result:?}", leech.get_ref().peer_addr().map_or_else(|e| format!("<error: {e}>"), |addr| addr.to_string()));
                
                WebSocket::send(leech, result);
            }
        });
        
        // Gives up the timeslice.
        thread::yield_now();
    }
}
