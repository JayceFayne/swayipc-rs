use super::common::receive_from_stream;
use super::socket::get_socketpath;
use crate::runtime::{sleep, AsyncWriteExt, Socket};
use crate::{CommandType::*, Error::SubscriptionFailed, *};
use serde::de::DeserializeOwned as Deserialize;
use std::io::ErrorKind::NotConnected;

#[derive(Debug)]
pub struct Connection(Socket);

impl Connection {
    pub async fn new() -> Fallible<Self> {
        let socketpath = get_socketpath().await;
        loop {
            let stream = Socket::connect(socketpath.as_path()).await;
            if matches!(stream.as_ref().map_err(|e| e.kind()), Err(NotConnected)) {
                sleep(100).await
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

    pub async fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<Fallible<()>>> {
        let outcome: Vec<CommandOutcome> =
            self.raw_command_with(RunCommand, payload.as_ref()).await?;
        Ok(outcome.into_iter().map(CommandOutcome::decode).collect())
    }

    pub async fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        self.raw_command(GetWorkspaces).await
    }

    pub async fn subscribe<T: AsRef<[EventType]>>(mut self, events: T) -> Fallible<EventStream> {
        let events = serde_json::ser::to_string(events.as_ref())?;
        let res: Success = self.raw_command_with(Subscribe, events.as_bytes()).await?;
        if !res.success {
            return Err(SubscriptionFailed(events));
        }
        Ok(EventStream::new(self.0))
    }

    pub async fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        self.raw_command(GetOutputs).await
    }

    pub async fn get_tree(&mut self) -> Fallible<Node> {
        self.raw_command(GetTree).await
    }

    pub async fn get_marks(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetMarks).await
    }

    pub async fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBarConfig).await
    }

    pub async fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        self.raw_command_with(GetBarConfig, id.as_ref()).await
    }

    pub async fn get_version(&mut self) -> Fallible<Version> {
        self.raw_command(GetVersion).await
    }

    pub async fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBindingModes).await
    }

    pub async fn get_config(&mut self) -> Fallible<Config> {
        self.raw_command(GetConfig).await
    }

    pub async fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        let res: Success = self.raw_command_with(SendTick, payload.as_ref()).await?;
        Ok(res.success)
    }

    pub async fn sync(&mut self) -> Fallible<bool> {
        let res: Success = self.raw_command(Sync).await?;
        Ok(res.success)
    }

    pub async fn get_binding_state(&mut self) -> Fallible<String> {
        let state: BindingState = self.raw_command(GetBindingState).await?;
        Ok(state.name)
    }

    pub async fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        self.raw_command(GetInputs).await
    }

    pub async fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        self.raw_command(GetSeats).await
    }
}

impl From<Socket> for Connection {
    fn from(s: Socket) -> Self {
        Self(s)
    }
}

impl From<Connection> for Socket {
    fn from(connection: Connection) -> Self {
        connection.0
    }
}
