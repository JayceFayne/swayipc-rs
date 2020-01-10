mod common;
mod connection;
#[cfg(not(feature = "event_stream"))]
mod event;
#[cfg(feature = "event_stream")]
mod event_stream;
#[cfg(test)]
mod tests;

pub use connection::Connection;
#[cfg(not(feature = "event_stream"))]
pub use event::EventIterator;
#[cfg(feature = "event_stream")]
pub use event_stream::EventStream;
