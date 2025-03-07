//!
//! Docker server utilities.
//!

/// Scale one or multiple replicated service.
pub fn scale(service: &'_ str, replicas: u32) {
    super::docker(["service", "scale", "--detach"], format!("{service}={replicas}"));
}
