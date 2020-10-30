use crate::{CommandType, Error::InvalidCommandType, Fallible};
use serde::de::DeserializeOwned as Deserialize;

impl CommandType {
    pub fn decode<D: Deserialize>(self, (payload_type, payload): (u32, Vec<u8>)) -> Fallible<D> {
        let command_type = u32::from(self);
        if payload_type != command_type {
            return Err(InvalidCommandType(payload_type, command_type));
        }
        Ok(serde_json::from_slice(&payload)?)
    }
}
