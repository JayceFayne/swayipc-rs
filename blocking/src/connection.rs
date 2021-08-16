use super::common::receive_from_stream;
use super::socket::get_socketpath;
use crate::{CommandType::*, Error::SubscriptionFailed, *};
use serde::de::DeserializeOwned as Deserialize;
use std::io::Write;
use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub struct Connection(UnixStream);

impl Connection {
    pub fn new() -> Fallible<Self> {
        let socketpath = get_socketpath();
        let unix_stream = UnixStream::connect(socketpath)?;
        Ok(Self(unix_stream))
    }

    fn raw_command<D: Deserialize>(&mut self, command_type: CommandType) -> Fallible<D> {
        self.0.write_all(command_type.encode().as_slice())?;
        command_type.decode(receive_from_stream(&mut self.0)?)
    }

    fn raw_command_with<D: Deserialize, T: AsRef<[u8]>>(
        &mut self,
        command_type: CommandType,
        payload: T,
    ) -> Fallible<D> {
        self.0
            .write_all(command_type.encode_with(payload).as_slice())?;
        command_type.decode(receive_from_stream(&mut self.0)?)
    }

    pub fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<Fallible<()>>> {
        let outcome: Vec<CommandOutcome> = self.raw_command_with(RunCommand, payload.as_ref())?;
        Ok(outcome.into_iter().map(CommandOutcome::decode).collect())
    }

    pub fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        self.raw_command(GetWorkspaces)
    }

    pub fn subscribe<T: AsRef<[EventType]>>(mut self, events: T) -> Fallible<EventStream> {
        let events = serde_json::ser::to_string(events.as_ref())?;
        let res: Success = self.raw_command_with(Subscribe, events.as_bytes())?;
        if !res.success {
            return Err(SubscriptionFailed(events));
        }
        Ok(EventStream::new(self.0))
    }

    pub fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        self.raw_command(GetOutputs)
    }

    pub fn get_tree(&mut self) -> Fallible<Node> {
        self.raw_command(GetTree)
    }

    pub fn get_marks(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetMarks)
    }

    pub fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBarConfig)
    }

    pub fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        self.raw_command_with(GetBarConfig, id.as_ref())
    }

    pub fn get_version(&mut self) -> Fallible<Version> {
        self.raw_command(GetVersion)
    }

    pub fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBindingModes)
    }

    pub fn get_config(&mut self) -> Fallible<Config> {
        self.raw_command(GetConfig)
    }

    pub fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        let res: Success = self.raw_command_with(SendTick, payload.as_ref())?;
        Ok(res.success)
    }

    pub fn sync(&mut self) -> Fallible<bool> {
        let res: Success = self.raw_command::<Success>(Sync)?;
        Ok(res.success)
    }

    pub fn get_binding_state(&mut self) -> Fallible<String> {
        let state: BindingState = self.raw_command(GetBindingState)?;
        Ok(state.name)
    }

    pub fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        self.raw_command(GetInputs)
    }

    pub fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        self.raw_command(GetSeats)
    }
}

impl From<UnixStream> for Connection {
    fn from(unix_stream: UnixStream) -> Self {
        Self(unix_stream)
    }
}

impl From<Connection> for UnixStream {
    fn from(connection: Connection) -> Self {
        connection.0
    }
}
