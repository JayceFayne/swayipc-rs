use super::common::receive_from_stream;
use crate::{Event, Fallible};
use std::os::unix::net::UnixStream;

pub struct EventStream(UnixStream);

impl EventStream {
    pub(super) fn new(stream: UnixStream) -> Self {
        Self(stream)
    }
}

impl Iterator for EventStream {
    type Item = Fallible<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(receive_from_stream(&mut self.0).and_then(Event::decode))
    }
}
