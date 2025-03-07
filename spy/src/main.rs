//!
//! Periodically check how all the workers in a Swarm are working,
//! broadcasting it throught a websocket.
//!
//! Port: 4000
//! Response: responding/total workers, i.e. two stringified [`u32`] separated by a `/`.
//!

use crate::websocket::WebSocket;
use spy::{self, docker};
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};


// A good spy spy its target.
#[path = "../../worker/src/websocket.rs"]
mod websocket;

const STACK: &str = "64";
const WORKERS: &str = "64_worker";
const NETWORK: &str = "64_default";

fn main() {
    ::ctrlc::set_handler(|| {
        exit(130);
    }).unwrap();
    
    let mut server = WebSocket::new(4000);
    let mut has_started = false;
    
    let mut deadline = Instant::now();
    const DELAY: Duration = Duration::from_secs(1);
    loop {
        // Wait a most 1s
        let now = Instant::now();
        if let Some(delay) = deadline.checked_duration_since(now) {
            thread::sleep(delay);
        }
        deadline = now + DELAY;
        
        // Ping all the servers, getting those who aren't responding
        println!();
        let (responding, workers) =
            docker::stack::ps(STACK).into_iter()
                .filter(|task| task.name.starts_with(WORKERS))
                .filter_map(|task|
                    docker::inspect(&task.id)
                        .and_then(|container| container.get_ip(NETWORK))
                        .map(|ip| (task.name, ip))
                )
                .map(|(name, ip)| {
                    println!("PING {name} ({ip})");
                    spy::ping(ip).is_ok() as u32
                })
                .fold((0_u32, 0_u32), |(sum, count), x| {
                    (sum + x, count + 1)
                });
        
        println!("Available: {responding}/{workers}");
        
        // Brodcast the data
        server.cleanse();
        server.accept();
        server.broadcast(format!("{responding}/{workers}"));
        
        // Avoid scaling up when workers are starting
        if responding > 0 {
            has_started = true;
        }
        
        // Scale up
        if responding == 0 && has_started {
            docker::service::scale(WORKERS, workers * 2);
        }
        
        // Scale down
        if workers >= 4 && responding > workers / 2 {
            docker::service::scale(WORKERS, workers / 2);
        }
    }
}
