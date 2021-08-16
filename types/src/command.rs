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
        IntoIterator::into_iter(crate::MAGIC)
            .chain(IntoIterator::into_iter(0_u32.to_ne_bytes()))
            .chain(IntoIterator::into_iter(u32::from(self).to_ne_bytes()))
            .collect()
    }

    pub fn encode_with<T: AsRef<[u8]>>(self, payload: T) -> Vec<u8> {
        let payload = payload.as_ref();
        IntoIterator::into_iter(crate::MAGIC)
            .chain(IntoIterator::into_iter(
                (payload.len() as u32).to_ne_bytes(),
            ))
            .chain(IntoIterator::into_iter(u32::from(self).to_ne_bytes()))
            .chain(payload.iter().cloned())
            .collect()
    }
}

impl From<CommandType> for u32 {
    fn from(value: CommandType) -> Self {
        value as u32
    }
}
