#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CommandType {
    /// Runs the payload as sway commands.
    RunCommand = 0,
    /// Get the list of current workspaces.
    GetWorkspaces = 1,
    /// Subscribe the IPC connection to the events listed in the payload.
    Subscribe = 2,
    /// Get the list of current outputs.
    GetOutputs = 3,
    /// Get the node layout tree.
    GetTree = 4,
    /// Get the names of all the marks currently set.
    GetMarks = 5,
    /// Get the specified bar config or a list of bar config names.
    GetBarConfig = 6,
    /// Get the version of sway that owns the IPC socket.
    GetVersion = 7,
    /// Get the list of binding mode names.
    GetBindingModes = 8,
    /// Returns the config that was last loaded.
    GetConfig = 9,
    /// Sends a tick event with the specified payload.
    SendTick = 10,
    /// Replies failure object for i3 compatibility.
    Sync = 11,
    /// Request the current binding state, e.g. the currently active binding
    /// mode name.
    GetBindingState = 12,
    /// Get the list of input devices.
    GetInputs = 100,
    /// Get the list of seats.
    GetSeats = 101,
}

impl CommandType {
    pub fn encode(self) -> Vec<u8> {
        crate::MAGIC
            .into_iter()
            .chain(0_u32.to_ne_bytes())
            .chain(u32::from(self).to_ne_bytes())
            .collect()
    }

    pub fn encode_with<T: AsRef<[u8]>>(self, payload: T) -> Vec<u8> {
        let payload = payload.as_ref();
        crate::MAGIC
            .into_iter()
            .chain((payload.len() as u32).to_ne_bytes())
            .chain(u32::from(self).to_ne_bytes())
            .chain(payload.iter().cloned())
            .collect()
    }
}

impl From<CommandType> for u32 {
    fn from(value: CommandType) -> Self {
        value as u32
    }
}
