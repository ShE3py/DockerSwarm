use crate::websocket::WebSocket;
use spy::stack;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};


// A good spy spy its target.
#[path = "../../worker/src/websocket.rs"]
mod websocket;

fn main() {
    ::ctrlc::set_handler(|| {
        exit(130);
    }).unwrap();
    
    let mut server = WebSocket::new(4000);
    let mut deadline = Instant::now();
    const DELAY: Duration = Duration::from_secs(1);
    loop {
        // Wait a most 1s
        let now = Instant::now();
        if let Some(delay) = deadline.checked_duration_since(now) {
            thread::sleep(delay);
        }
        deadline = now + DELAY;
        
        println!();
        let (working, workers) =
            stack::ps("64").into_iter()
                .filter(|task| task.name.starts_with("64_worker"))
                .filter_map(|task|
                    spy::inspect(&task.id)
                        .and_then(|container| container.get_ip("64_default"))
                        .map(|ip| (task.name, ip))
                )
                .map(|(name, ip)| {
                    println!("PING {name} ({ip})");
                    spy::ping(ip).is_ok() as u32
                })
                .fold((0_u32, 0_u32), |(sum, count), x| {
                    (sum + x, count + 1)
                });
        
        println!("Available: {working}/{workers}");
        
        server.cleanse();
        server.accept();
        server.broadcast(format!("{working}/{workers}"));
    }
}
