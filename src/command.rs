use crate::{ensure, receive_from_stream, Connection, Fallible, MAGIC};
use byteorder::{WriteBytesExt, LE};
use serde::de::DeserializeOwned;
use serde_json::from_slice;
use std::io::Write;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum CommandType {
    RunCommand,
    GetWorkspaces,
    Subscribe,
    GetOutputs,
    GetTree,
    GetMarks,
    GetBarConfig,
    GetVersion,
    GetBindingModes,
    GetConfig,
    SendTick,
    Sync,
    GetInputs,
    GetSeats,
}

impl From<CommandType> for u32 {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::RunCommand => 0,
            CommandType::GetWorkspaces => 1,
            CommandType::Subscribe => 2,
            CommandType::GetOutputs => 3,
            CommandType::GetTree => 4,
            CommandType::GetMarks => 5,
            CommandType::GetBarConfig => 6,
            CommandType::GetVersion => 7,
            CommandType::GetBindingModes => 8,
            CommandType::GetConfig => 9,
            CommandType::SendTick => 10,
            CommandType::Sync => 11,
            CommandType::GetInputs => 100,
            CommandType::GetSeats => 101,
        }
    }
}

impl From<u32> for CommandType {
    fn from(value: u32) -> Self {
        match value {
            0 => CommandType::RunCommand,
            1 => CommandType::GetWorkspaces,
            2 => CommandType::Subscribe,
            3 => CommandType::GetOutputs,
            4 => CommandType::GetTree,
            5 => CommandType::GetMarks,
            6 => CommandType::GetBarConfig,
            7 => CommandType::GetVersion,
            8 => CommandType::GetBindingModes,
            9 => CommandType::GetConfig,
            10 => CommandType::SendTick,
            11 => CommandType::Sync,
            100 => CommandType::GetInputs,
            101 => CommandType::GetSeats,
            _ => unimplemented!("command '{}'", value),
        }
    }
}

impl Connection {
    pub(crate) fn send_raw_command_with_payload<P: AsRef<str>>(
        &mut self,
        message_type: CommandType,
        payload: P,
    ) -> Fallible<()> {
        let payload = payload.as_ref().bytes();
        let len = payload.len();
        let mut bytes = Vec::with_capacity(14 + len);
        bytes.extend(MAGIC.iter());
        bytes.write_u32::<LE>(len as u32)?;
        bytes.write_u32::<LE>(message_type.into())?;
        bytes.extend(payload);
        self.stream.write_all(&bytes[..])?;
        Ok(())
    }

    pub(crate) fn send_raw_command(&mut self, message_type: CommandType) -> Fallible<()> {
        let mut bytes = Vec::with_capacity(14);
        bytes.extend(MAGIC.iter());
        bytes.write_u32::<LE>(0)?;
        bytes.write_u32::<LE>(message_type.into())?;
        self.stream.write_all(&bytes[..])?;
        Ok(())
    }

    pub(crate) fn receive_raw_command<T: DeserializeOwned>(
        &mut self,
        command_type: CommandType,
    ) -> Fallible<T> {
        let (message_type, payload) = receive_from_stream(&mut self.stream)?;
        ensure!(
            u32::from(command_type) == message_type,
            "did receive a message with another type than requested"
        );
        Ok(from_slice(&payload)?)
    }

    pub(crate) fn send_receive_raw_command_with_payload<T: DeserializeOwned, P: AsRef<str>>(
        &mut self,
        message_type: CommandType,
        payload: P,
    ) -> Fallible<T> {
        self.send_raw_command_with_payload(message_type, payload)?;
        Ok(self.receive_raw_command(message_type)?)
    }

    pub(crate) fn send_receive_raw_command<T: DeserializeOwned>(
        &mut self,
        message_type: CommandType,
    ) -> Fallible<T> {
        self.send_raw_command(message_type)?;
        Ok(self.receive_raw_command(message_type)?)
    }
}
