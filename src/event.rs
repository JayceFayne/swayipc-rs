use crate::reply::Event;
use crate::{bail, receive_from_stream, Fallible};
use serde_derive::Serialize;
use serde_json::from_slice;
use std::os::unix::net::UnixStream;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Workspace,
    Mode,
    Window,
    #[serde(rename = "barconfig_update")]
    BarConfigUpdate,
    Binding,
    Shutdown,
    Tick,
    BarStatusUpdate,
    Input,
}

pub struct EventIterator(UnixStream);

impl EventIterator {
    pub(crate) fn new(stream: UnixStream) -> EventIterator {
        Self(stream)
    }

    fn receive_event(&mut self) -> Fallible<Event> {
        let (event_type, payload) = receive_from_stream(&mut self.0)?;
        Ok(
            // strip the highest order bit indicating it's an event
            match (event_type << 1) >> 1 {
                0 => Event::Workspace(from_slice(&payload)?),
                2 => Event::Mode(from_slice(&payload)?),
                3 => Event::Window(from_slice(&payload)?),
                4 => Event::BarConfigUpdate(from_slice(&payload)?),
                5 => Event::Binding(from_slice(&payload)?),
                6 => Event::Shutdown(from_slice(&payload)?),
                7 => Event::Tick(from_slice(&payload)?),
                14 => Event::BarStatusUpdate(from_slice(&payload)?),
                15 => Event::Input(from_slice(&payload)?),
                _ => bail!("received event {} we didnt't subscribed to", event_type),
            },
        )
    }
}

impl Iterator for EventIterator {
    type Item = Fallible<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.receive_event())
    }
}
