#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

mod command;
#[cfg(feature = "error")]
mod error;
mod event;
mod reply;
mod utils;

pub use command::CommandType;
#[cfg(feature = "error")]
pub use error::{Error, Fallible};
pub use event::EventType;
pub use reply::*;

pub const MAGIC: [u8; 6] = [105, 51, 45, 105, 112, 99];
