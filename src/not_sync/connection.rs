use super::common::receive_from_stream;
use crate::command::CommandType;
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

    pub(crate) async fn raw_command<D: Deserialize>(
        &mut self,
        command_type: CommandType,
        payload: Option<&str>,
    ) -> Fallible<D> {
        self.0.write_all(&command_type.encode(payload)).await?;
        let (message_type, payload) = receive_from_stream(&mut self.0).await?;
        ensure!(
            u32::from(command_type) == message_type,
            "did receive a reply with another type than requested"
        );
        Ok(from_slice(&payload)?)
    }

    pub async fn run_command<T: AsRef<str>>(
        &mut self,
        payload: T,
    ) -> Fallible<Vec<CommandOutcome>> {
        Ok(self
            .raw_command(CommandType::RunCommand, Some(payload.as_ref()))
            .await?)
    }

    pub async fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        Ok(self.raw_command(CommandType::GetWorkspaces, None).await?)
    }

    pub async fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
        ensure!(
            self.raw_command::<Success>(
                CommandType::Subscribe,
                Some(&serde_json::ser::to_string(events)?)
            )
            .await?
            .success,
            "failed to subscribe to events"
        );
        Ok(EventIterator(self.0))
    }

    pub async fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        Ok(self.raw_command(CommandType::GetOutputs, None).await?)
    }

    pub async fn get_tree(&mut self) -> Fallible<Node> {
        Ok(self.raw_command(CommandType::GetTree, None).await?)
    }

    pub async fn get_marks(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetMarks, None).await?)
    }

    pub async fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetBarConfig, None).await?)
    }

    pub async fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        Ok(self
            .raw_command(CommandType::GetBarConfig, Some(id.as_ref()))
            .await?)
    }

    pub async fn get_version(&mut self) -> Fallible<Version> {
        Ok(self.raw_command(CommandType::GetVersion, None).await?)
    }

    pub async fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        Ok(self.raw_command(CommandType::GetBindingModes, None).await?)
    }

    pub async fn get_config(&mut self) -> Fallible<Config> {
        Ok(self.raw_command(CommandType::GetConfig, None).await?)
    }

    pub async fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(CommandType::SendTick, Some(payload.as_ref()))
            .await?
            .success)
    }

    pub async fn send_sync(&mut self) -> Fallible<bool> {
        Ok(self
            .raw_command::<Success>(CommandType::Sync, None)
            .await?
            .success)
    }

    pub async fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        Ok(self.raw_command(CommandType::GetInputs, None).await?)
    }

    pub async fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        Ok(self.raw_command(CommandType::GetSeats, None).await?)
    }
}
