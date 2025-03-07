//!
//! Private utilities. Proxy for neat `docker::` naming!
//!

use std::ffi::OsStr;
use std::process::Command;

mod inspect;
pub mod service;
pub mod stack;

pub use inspect::*;

/// Returns the stdout of a Docker command, or an empty vec in case of failure.
pub fn docker<const N: usize>(args: [&'static str; N], arg: impl AsRef<OsStr>) -> Vec<u8> {
    let mut cmd = Command::new("docker");
    cmd.args(args).arg(arg);
    
    match cmd.output() {
        // FIXME: use [`ExitStatus::exit_ok`] when stable
        Ok(output) if output.status.success() => output.stdout,
        Ok(output) => {
            eprintln!();
            eprintln!("error: child exited unsuccessfully: {}", output.status);
            eprintln!("-- command --\n{cmd:?}");
            if !output.stdout.is_empty() {
                eprintln!("-- stdout --\n{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("-- stderr --\n{}", String::from_utf8_lossy(&output.stderr));
            }
            Vec::new()
        },
        Err(e) => {
            eprintln!();
            eprintln!("error: child failed to spawn: {}", e);
            eprintln!("-- command --\n{cmd:?}");
            Vec::new()
        }
    }
}
