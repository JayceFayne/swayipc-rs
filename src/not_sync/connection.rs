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
        let (reply_type, payload) = receive_from_stream(&mut self.0).await?;
        let command_type = u32::from(command_type);
        ensure!(
            command_type == reply_type,
            "did receive a reply with type '{}' but send command with type '{}'",
            reply_type,
            command_type
        );
        Ok(from_slice(&payload)?)
    }

    pub async fn run_command<T: AsRef<str>>(
        &mut self,
        payload: T,
    ) -> Fallible<Vec<CommandOutcome>> {
        self.raw_command(CommandType::RunCommand, Some(payload.as_ref()))
            .await
    }

    pub async fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        self.raw_command(CommandType::GetWorkspaces, None).await
    }

    pub async fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
        let events = serde_json::ser::to_string(events)?;
        ensure!(
            self.raw_command::<Success>(CommandType::Subscribe, Some(&events))
                .await?
                .success,
            "failed to subscribe to events '{}'",
            events
        );
        Ok(EventIterator(self.0))
    }

    pub async fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        self.raw_command(CommandType::GetOutputs, None).await
    }

    pub async fn get_tree(&mut self) -> Fallible<Node> {
        self.raw_command(CommandType::GetTree, None).await
    }

    pub async fn get_marks(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(CommandType::GetMarks, None).await
    }

    pub async fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(CommandType::GetBarConfig, None).await
    }

    pub async fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        self.raw_command(CommandType::GetBarConfig, Some(id.as_ref()))
            .await
    }

    pub async fn get_version(&mut self) -> Fallible<Version> {
        self.raw_command(CommandType::GetVersion, None).await
    }

    pub async fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(CommandType::GetBindingModes, None).await
    }

    pub async fn get_config(&mut self) -> Fallible<Config> {
        self.raw_command(CommandType::GetConfig, None).await
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
        self.raw_command(CommandType::GetInputs, None).await
    }

    pub async fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        self.raw_command(CommandType::GetSeats, None).await
    }
}
