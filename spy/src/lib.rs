//!
//! Library for retrieving the workers' statuses.
//!

pub mod docker;
mod ping;

pub use ping::ping;
