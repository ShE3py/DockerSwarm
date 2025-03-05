use spy::stack;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() {
    ::ctrlc::set_handler(|| {
        exit(130);
    }).unwrap();
    
    loop {
        thread::sleep(Duration::from_secs(5));
        
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
    }
}
