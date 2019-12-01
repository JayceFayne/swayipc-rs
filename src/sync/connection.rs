use super::common::receive_from_stream;
use crate::command::CommandType;
use crate::reply::{
    BarConfig, CommandOutcome, Config, Input, Node, Output, Seat, Success, Version, Workspace,
};
use crate::socket::get_path;
use crate::{ensure, EventIterator, EventType, Fallible};
use serde::de::DeserializeOwned as Deserialize;
use serde_json::from_slice;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct Connection(UnixStream);

impl Connection {
    pub fn new() -> Fallible<Self> {
        Ok(Self(UnixStream::connect(get_path()?)?))
    }

    pub(crate) fn raw_command<D: Deserialize>(
        &mut self,
        command_type: CommandType,
        payload: Option<&str>,
    ) -> Fallible<D> {
        self.0.write_all(&command_type.encode(payload))?;
        let (message_type, payload) = receive_from_stream(&mut self.0)?;
        ensure!(
            u32::from(command_type) == message_type,
            "did receive a reply with another type than requested"
        );
        Ok(from_slice(&payload)?)
    }

    pub fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<CommandOutcome>> {
        Ok(self.raw_command(CommandType::RunCommand, Some(payload.as_ref()))?)
    }

    pub fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        Ok(self.raw_command(CommandType::GetWorkspaces, None)?)
    }

    pub fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
        ensure!(
            self.raw_command::<Success>(
                CommandType::Subscribe,
                Some(&serde_json::ser::to_string(events)?)
            )?
            .success,
            "failed to subscribe to events"
        );
        Ok(EventIterator(self.0))
    }

    pub fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        Ok(self.raw_command(CommandType::GetOutputs, None)?)
    }

    pub fn get_tree(&mut self) -> Fallible<Node> {
        Ok(self.raw_command(CommandType::GetTree, None)?)
    }

    pub fn get_marks(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetMarks, None)?)
    }

    pub fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetBarConfig, None)?)
    }

    pub fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        Ok(self.raw_command(CommandType::GetBarConfig, Some(id.as_ref()))?)
    }

    pub fn get_version(&mut self) -> Fallible<Version> {
        Ok(self.raw_command(CommandType::GetVersion, None)?)
    }

    pub fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetBindingModes, None)?)
    }

    pub fn get_config(&mut self) -> Fallible<Config> {
        Ok(self.raw_command(CommandType::GetConfig, None)?)
    }

    pub fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(CommandType::SendTick, Some(payload.as_ref()))?
            .success)
    }

    pub fn send_sync(&mut self) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(CommandType::Sync, None)?
            .success)
    }

    pub fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        Ok(self.raw_command(CommandType::GetInputs, None)?)
    }

    pub fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        Ok(self.raw_command(CommandType::GetSeats, None)?)
    }
}
