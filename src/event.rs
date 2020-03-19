use crate::reply::Event;
use crate::{bail, Error};
use serde::Serialize;
use serde_json::from_slice;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize)]
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
    BarStateUpdate,
    Input,
}

impl TryFrom<(u32, Vec<u8>)> for Event {
    type Error = Error;

    fn try_from((event_type, payload): (u32, Vec<u8>)) -> Result<Self, Self::Error> {
        use Event::*;
        // strip the highest order bit indicating it's an event
        // since we dont convert to hex we also dont match on the (hex) values written in the sway-ipc docs!
        let event = (event_type << 1) >> 1;
        Ok(match event {
            0 => Workspace(from_slice(&payload)?),
            2 => Mode(from_slice(&payload)?),
            3 => Window(from_slice(&payload)?),
            4 => BarConfigUpdate(from_slice(&payload)?),
            5 => Binding(from_slice(&payload)?),
            6 => Shutdown(from_slice(&payload)?),
            7 => Tick(from_slice(&payload)?),
            20 => BarStateUpdate(from_slice(&payload)?),
            21 => Input(from_slice(&payload)?),
            _ => bail!("received unimplemented event '{}'", event),
        })
    }
}
