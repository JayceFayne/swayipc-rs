use super::common::receive_from_stream;
use crate::command::{Command, CommandType};
use crate::reply::{
    BarConfig, CommandOutcome, Config, Input, Node, Output, Seat, Success, Version, Workspace,
};
use crate::socket::get_path;
use crate::{ensure, EventIterator, EventType, Fallible};
use async_std::io::prelude::WriteExt;
use async_std::os::unix::net::UnixStream;
use serde::de::DeserializeOwned as Deserialize;
use serde_json::from_slice;

pub struct Connection(UnixStream);

impl Connection {
    pub async fn new() -> Fallible<Self> {
        Ok(Self(UnixStream::connect(get_path()?).await?))
    }

    pub(crate) async fn raw_command<T: Deserialize>(&mut self, com: Command<'_>) -> Fallible<T> {
        self.0.write_all(&com.encode()).await?;
        let (message_type, payload) = receive_from_stream(&mut self.0).await?;
        ensure!(
            u32::from(com.command_type) == message_type,
            "did receive a reply with another type than requested"
        );
        Ok(from_slice(&payload)?)
    }

    pub async fn run_command<T: AsRef<str>>(
        &mut self,
        payload: T,
    ) -> Fallible<Vec<CommandOutcome>> {
        Ok(self
            .raw_command(command_new!(CommandType::RunCommand, payload))
            .await?)
    }

    pub async fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetWorkspaces))
            .await?)
    }

    pub async fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
        ensure!(
            self.raw_command::<Success>(command_new!(
                CommandType::Subscribe,
                serde_json::ser::to_string(events)?
            ))
            .await?
            .success,
            "failed to subscribe to events"
        );
        Ok(EventIterator(self.0))
    }

    pub async fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetOutputs))
            .await?)
    }

    pub async fn get_tree(&mut self) -> Fallible<Node> {
        Ok(self.raw_command(command_new!(CommandType::GetTree)).await?)
    }

    pub async fn get_marks(&mut self) -> Fallible<Vec<String>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetMarks))
            .await?)
    }

    pub async fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetBarConfig))
            .await?)
    }

    pub async fn get_bar_config(&mut self, id: &str) -> Fallible<BarConfig> {
        Ok(self
            .raw_command(command_new!(CommandType::GetBarConfig, id))
            .await?)
    }

    pub async fn get_version(&mut self) -> Fallible<Version> {
        Ok(self
            .raw_command(command_new!(CommandType::GetVersion))
            .await?)
    }

    pub async fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetBindingModes))
            .await?)
    }

    pub async fn get_config(&mut self) -> Fallible<Config> {
        Ok(self
            .raw_command(command_new!(CommandType::GetConfig))
            .await?)
    }

    pub async fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(command_new!(CommandType::SendTick, payload))
            .await?
            .success)
    }

    pub async fn send_sync(&mut self) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(command_new!(CommandType::Sync))
            .await?
            .success)
    }

    pub async fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetInputs))
            .await?)
    }

    pub async fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        Ok(self
            .raw_command(command_new!(CommandType::GetSeats))
            .await?)
    }
}
