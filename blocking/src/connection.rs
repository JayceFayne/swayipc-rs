use super::common::receive_from_stream;
use super::socket::get_socketpath;
use crate::{CommandType::*, Error::SubscriptionFailed, *};
use serde::de::DeserializeOwned as Deserialize;
use std::io::Write;
use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub struct Connection(UnixStream);

impl Connection {
    /// Creates a new `Connection` to sway-ipc.
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

    /// Runs the payload as sway commands.
    pub fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<Fallible<()>>> {
        let outcome: Vec<CommandOutcome> = self.raw_command_with(RunCommand, payload.as_ref())?;
        Ok(outcome.into_iter().map(CommandOutcome::decode).collect())
    }

    /// Get the list of current workspaces.
    pub fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        self.raw_command(GetWorkspaces)
    }

    /// Subscribe the IPC connection to the events listed in the payload.
    pub fn subscribe<T: AsRef<[EventType]>>(mut self, events: T) -> Fallible<EventStream> {
        let events = serde_json::ser::to_string(events.as_ref())?;
        let res: Success = self.raw_command_with(Subscribe, events.as_bytes())?;
        if !res.success {
            return Err(SubscriptionFailed(events));
        }
        Ok(EventStream::new(self.0))
    }

    /// Get the list of current outputs.
    pub fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        self.raw_command(GetOutputs)
    }

    /// Get the node layout tree.
    pub fn get_tree(&mut self) -> Fallible<Node> {
        self.raw_command(GetTree)
    }

    /// Get the names of all the marks currently set.
    pub fn get_marks(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetMarks)
    }

    /// Get a list of bar config names.
    pub fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBarConfig)
    }

    /// Get the specified bar config.
    pub fn get_bar_config<T: AsRef<str>>(&mut self, id: T) -> Fallible<BarConfig> {
        self.raw_command_with(GetBarConfig, id.as_ref())
    }

    /// Get the version of sway that owns the IPC socket.
    pub fn get_version(&mut self) -> Fallible<Version> {
        self.raw_command(GetVersion)
    }

    /// Get the list of binding mode names.
    pub fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        self.raw_command(GetBindingModes)
    }

    /// Returns the config that was last loaded.
    pub fn get_config(&mut self) -> Fallible<Config> {
        self.raw_command(GetConfig)
    }

    /// Sends a tick event with the specified payload.
    pub fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        let res: Success = self.raw_command_with(SendTick, payload.as_ref())?;
        Ok(res.success)
    }

    /// Replies failure object for i3 compatibility.
    pub fn sync(&mut self) -> Fallible<bool> {
        let res: Success = self.raw_command::<Success>(Sync)?;
        Ok(res.success)
    }

    /// Request the current binding state, e.g.  the currently active binding
    /// mode name.
    pub fn get_binding_state(&mut self) -> Fallible<String> {
        let state: BindingState = self.raw_command(GetBindingState)?;
        Ok(state.name)
    }

    /// Get the list of input devices.
    pub fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        self.raw_command(GetInputs)
    }

    /// Get the list of seats.
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
