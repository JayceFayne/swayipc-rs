use crate::{
    Error::UnimplementedEvent,
    Event::{self, *},
    Fallible,
};

impl Event {
    pub fn decode((payload_type, payload): (u32, Vec<u8>)) -> Fallible<Self> {
        // strip the highest order bit indicating it's an event
        // since we dont convert to hex we also dont match on the (hex) values written in the sway-ipc docs!
        let event_type = (payload_type << 1) >> 1;
        Ok(match event_type {
            0 => Workspace(serde_json::from_slice(&payload)?),
            2 => Mode(serde_json::from_slice(&payload)?),
            3 => Window(serde_json::from_slice(&payload)?),
            4 => BarConfigUpdate(serde_json::from_slice(&payload)?),
            5 => Binding(serde_json::from_slice(&payload)?),
            6 => Shutdown(serde_json::from_slice(&payload)?),
            7 => Tick(serde_json::from_slice(&payload)?),
            20 => BarStateUpdate(serde_json::from_slice(&payload)?),
            21 => Input(serde_json::from_slice(&payload)?),
            _ => return Err(UnimplementedEvent(event_type, payload)),
        })
    }
}
