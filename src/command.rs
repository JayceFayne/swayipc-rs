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
            CommandType::RunCommand => 0,
            CommandType::GetWorkspaces => 1,
            CommandType::Subscribe => 2,
            CommandType::GetOutputs => 3,
            CommandType::GetTree => 4,
            CommandType::GetMarks => 5,
            CommandType::GetBarConfig => 6,
            CommandType::GetVersion => 7,
            CommandType::GetBindingModes => 8,
            CommandType::GetConfig => 9,
            CommandType::SendTick => 10,
            CommandType::Sync => 11,
            CommandType::GetInputs => 100,
            CommandType::GetSeats => 101,
        }
    }
}

impl From<u32> for CommandType {
    fn from(value: u32) -> Self {
        match value {
            0 => CommandType::RunCommand,
            1 => CommandType::GetWorkspaces,
            2 => CommandType::Subscribe,
            3 => CommandType::GetOutputs,
            4 => CommandType::GetTree,
            5 => CommandType::GetMarks,
            6 => CommandType::GetBarConfig,
            7 => CommandType::GetVersion,
            8 => CommandType::GetBindingModes,
            9 => CommandType::GetConfig,
            10 => CommandType::SendTick,
            11 => CommandType::Sync,
            100 => CommandType::GetInputs,
            101 => CommandType::GetSeats,
            _ => unimplemented!("command '{}'", value),
        }
    }
}

pub(crate) struct Command<'a> {
    pub command_type: CommandType,
    pub payload: Option<&'a str>,
}

impl Command<'_> {
    pub fn encode(&self) -> Vec<u8> {
        if let Some(payload) = self.payload {
            let payload = payload.bytes();
            let len = payload.len();
            let mut bytes = Vec::with_capacity(14 + len);
            bytes.extend(crate::MAGIC.iter());
            bytes.extend(&(len as u32).to_ne_bytes());
            bytes.extend(&u32::from(self.command_type).to_ne_bytes());
            bytes.extend(payload);
            bytes
        } else {
            let mut bytes = Vec::with_capacity(14);
            bytes.extend(crate::MAGIC.iter());
            bytes.extend(&0_u32.to_ne_bytes());
            bytes.extend(&u32::from(self.command_type).to_ne_bytes());
            bytes
        }
    }
}

macro_rules! command_new {
    ($t:expr) => {
        Command {
            command_type: $t,
            payload: None,
        }
    };
    ($t:expr, $p:expr) => {
        Command {
            command_type: $t,
            payload: Some($p.as_ref()),
        }
    };
}
