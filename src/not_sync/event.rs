use super::common::receive_from_stream;
use crate::reply::Event;
use crate::Fallible;
use async_std::os::unix::net::UnixStream;
use std::convert::TryFrom;

pub struct EventIterator(pub(crate) UnixStream);

impl EventIterator {
    #[deprecated(note = "use the `event_stream` feature instead")]
    pub async fn next(&mut self) -> Fallible<Event> {
        Event::try_from(receive_from_stream(&mut self.0).await?)
    }
}
