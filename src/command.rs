pub(crate) use CommandType::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum CommandType {
    RunCommand,
    GetWorkspaces,
    Subscribe,
    GetOutputs,
    GetTree,
    GetMarks,
    GetBarConfig,
    GetVersion,
    GetBindingModes,
    GetConfig,
    SendTick,
    Sync,
    GetInputs,
    GetSeats,
}

impl From<CommandType> for u32 {
    fn from(value: CommandType) -> Self {
        match value {
            RunCommand => 0,
            GetWorkspaces => 1,
            Subscribe => 2,
            GetOutputs => 3,
            GetTree => 4,
            GetMarks => 5,
            GetBarConfig => 6,
            GetVersion => 7,
            GetBindingModes => 8,
            GetConfig => 9,
            SendTick => 10,
            Sync => 11,
            GetInputs => 100,
            GetSeats => 101,
        }
    }
}

impl CommandType {
    pub fn encode(self, payload: Option<&str>) -> Vec<u8> {
        if let Some(payload) = payload {
            let payload = payload.bytes();
            let len = payload.len();
            let mut bytes = Vec::with_capacity(14 + len);
            bytes.extend(crate::MAGIC.iter());
            bytes.extend(&(len as u32).to_ne_bytes());
            bytes.extend(&u32::from(self).to_ne_bytes());
            bytes.extend(payload);
            bytes
        } else {
            let mut bytes = Vec::with_capacity(14);
            bytes.extend(crate::MAGIC.iter());
            bytes.extend(&0_u32.to_ne_bytes());
            bytes.extend(&u32::from(self).to_ne_bytes());
            bytes
        }
    }
}
