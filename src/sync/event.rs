use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use std::convert::TryFrom;
use std::os::unix::net::UnixStream;

pub struct EventIterator(UnixStream);

impl EventIterator {
    pub(crate) fn new(stream: UnixStream) -> EventIterator {
        Self(stream)
    }

    fn receive_event(&mut self) -> Fallible<Event> {
        Event::try_from(receive_from_stream(&mut self.0)?)
    }
}

impl Iterator for EventIterator {
    type Item = Fallible<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.receive_event())
    }
}
