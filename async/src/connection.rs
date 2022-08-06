use super::common::receive_from_stream;
use super::socket::get_socketpath;
use crate::{CommandType::*, Error::SubscriptionFailed, *};
use async_io::{Async, Timer};
use futures_lite::AsyncWriteExt;
use serde::de::DeserializeOwned as Deserialize;
use std::io::ErrorKind::NotConnected;
use std::os::unix::net::UnixStream;
use std::time::Duration;

#[derive(Debug)]
pub struct Connection(Async<UnixStream>);

impl Connection {
    /// Creates a new async `Connection` to sway-ipc.
    pub async fn new() -> Fallible<Self> {
        let socketpath = get_socketpath().await;
        loop {
            let stream = Async::<UnixStream>::connect(socketpath.as_path()).await;
            if matches!(stream.as_ref().map_err(|e| e.kind()), Err(NotConnected)) {
                Timer::after(Duration::from_millis(100)).await;
            } else {
                return Ok(Self(stream?));
            }
        }
    }

    async fn raw_command<D: Deserialize>(&mut self, command_type: CommandType) -> Fallible<D> {
        self.0.write_all(command_type.encode().as_slice()).await?;
        command_type.decode(receive_from_stream(&mut self.0).await?)
    }

    async fn raw_command_with<D: Deserialize, T: AsRef<[u8]>>(
        &mut self,
        command_type: CommandType,
        payload: T,
    ) -> Fallible<D> {
        self.0
            .write_all(command_type.encode_with(payload).as_slice())
            .await?;
        command_type.decode(receive_from_stream(&mut self.0).await?)
    }

    /// Runs the payload as sway commands.
    pub async fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<Fallible<()>>> {
        let outcome: Vec<CommandOutcome> =
            self.raw_command_with(RunCommand, payload.as_ref()).await?;
        Ok(outcome.into_iter().map(CommandOutcome::decode).collect())
    }

    /// Get the list of current workspaces.
    pub async fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        self.raw_command(GetWorkspaces).await
    }

    /// Subscribe the IPC connection to the events listed in the payload.
    pub async fn subscribe<T: AsRef<[EventType]>>(mut self, events: T) -> Fallible<EventStream> {
        let events = serde_json::ser::to_string(events.as_ref())?;
        let res: Success = self.raw_command_with(Subscribe, events.as_bytes()).await?;
        if !res.success {
            return Err(SubscriptionFailed(events));
        }
        Ok(EventStream::new(self.0))
    }

    /// Get the list of current outputs.
    pub async fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        self.raw_command(GetOutputs).await
    }

    /// Get the node layout tree.
    pub async fn get_tree(&mut self) -> Fallible<Node> {
        self.raw_command(GetTree).await
    }

    /// Get the names of all the marks currently set.
    pub async fn get_marks(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetMarks).await
    }

    /// Get a list of bar config names.
    pub async fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBarConfig).await
    }

    /// Get the specified bar config.
    pub async fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        self.raw_command_with(GetBarConfig, id.as_ref()).await
    }

    /// Get the version of sway that owns the IPC socket.
    pub async fn get_version(&mut self) -> Fallible<Version> {
        self.raw_command(GetVersion).await
    }

    /// Get the list of binding mode names.
    pub async fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBindingModes).await
    }

    /// Returns the config that was last loaded.
    pub async fn get_config(&mut self) -> Fallible<Config> {
        self.raw_command(GetConfig).await
    }

    /// Sends a tick event with the specified payload.
    pub async fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        let res: Success = self.raw_command_with(SendTick, payload.as_ref()).await?;
        Ok(res.success)
    }

    /// Replies failure object for i3 compatibility.
    pub async fn sync(&mut self) -> Fallible<bool> {
        let res: Success = self.raw_command(Sync).await?;
        Ok(res.success)
    }

    /// Request the current binding state, e.g.  the currently active binding
    /// mode name.
    pub async fn get_binding_state(&mut self) -> Fallible<String> {
        let state: BindingState = self.raw_command(GetBindingState).await?;
        Ok(state.name)
    }

    /// Get the list of input devices.
    pub async fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        self.raw_command(GetInputs).await
    }

    /// Get the list of seats.
    pub async fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        self.raw_command(GetSeats).await
    }
}

impl From<Async<UnixStream>> for Connection {
    fn from(unix_stream: Async<UnixStream>) -> Self {
        Self(unix_stream)
    }
}

impl From<Connection> for Async<UnixStream> {
    fn from(connection: Connection) -> Self {
        connection.0
    }
}
