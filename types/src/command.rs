#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CommandType {
    RunCommand = 0,
    GetWorkspaces = 1,
    Subscribe = 2,
    GetOutputs = 3,
    GetTree = 4,
    GetMarks = 5,
    GetBarConfig = 6,
    GetVersion = 7,
    GetBindingModes = 8,
    GetConfig = 9,
    SendTick = 10,
    Sync = 11,
    GetBindingState = 12,
    GetInputs = 100,
    GetSeats = 101,
}

impl CommandType {
    pub fn encode(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(14);
        bytes.extend(crate::MAGIC.iter());
        bytes.extend(0_u32.to_ne_bytes().iter());
        bytes.extend(u32::from(self).to_ne_bytes().iter());
        bytes
    }

    pub fn encode_with(self, payload: &str) -> Vec<u8> {
        let payload = payload.bytes();
        let len = payload.len();
        let mut bytes = Vec::with_capacity(14 + len);
        bytes.extend(crate::MAGIC.iter());
        bytes.extend((len as u32).to_ne_bytes().iter());
        bytes.extend(u32::from(self).to_ne_bytes().iter());
        bytes.extend(payload);
        bytes
    }
}

impl From<CommandType> for u32 {
    fn from(value: CommandType) -> Self {
        value as u32
    }
}
