use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use std::convert::TryFrom;
use std::os::unix::net::UnixStream;

pub struct EventIterator(pub(crate) UnixStream);

impl Iterator for EventIterator {
    type Item = Fallible<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match receive_from_stream(&mut self.0) {
            Ok(v) => Event::try_from(v),
            Err(e) => Err(e),
        })
    }
}
