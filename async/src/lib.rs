#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

mod common;
mod connection;
mod event;
mod socket;
#[cfg(test)]
mod tests;

pub use connection::Connection;
pub use event::EventStream;
pub use swayipc_types::*;
