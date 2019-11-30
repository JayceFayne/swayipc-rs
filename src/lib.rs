mod command;
mod event;
pub mod reply;
mod socket;
#[cfg(test)]
mod tests;
mod utils;

use command::CommandType;
pub use event::{EventIterator, EventType};
pub use failure::{bail, ensure, Error, Fallible};
use reply::{
    BarConfig, CommandOutcome, Config, Input, Node, Output, Seat, Success, Version, Workspace,
};
use std::os::unix::net::UnixStream;

const MAGIC: [u8; 6] = [105, 51, 45, 105, 112, 99];

pub(crate) fn receive_from_stream(stream: &mut UnixStream) -> Fallible<(u32, Vec<u8>)> {
    use byteorder::{ReadBytesExt, LE};
    use std::io::Read;
    let mut magic_data = [0_u8; 6];
    stream.read_exact(&mut magic_data)?;
    ensure!(
        magic_data == MAGIC,
        "unexpected magic string: expected 'i3-ipc' but got {}",
        String::from_utf8_lossy(&magic_data)
    );
    let payload_len = stream.read_u32::<LE>()?;
    let message_type = stream.read_u32::<LE>()?;
    let mut payload_data = vec![0_u8; payload_len as usize];
    stream.read_exact(&mut payload_data[..])?;
    Ok((message_type, payload_data))
}

pub struct Connection(UnixStream);

impl Connection {
    pub fn new() -> Fallible<Self> {
        Ok(Self(UnixStream::connect(socket::get_path()?)?))
    }

    pub fn run_command<T: AsRef<str>>(&mut self, command: T) -> Fallible<Vec<CommandOutcome>> {
        Ok(self.send_receive_raw_command_with_payload(CommandType::RunCommand, command)?)
    }

    pub fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
        Ok(self.send_receive_raw_command(CommandType::GetWorkspaces)?)
    }

    pub fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
        ensure!(
            self.send_receive_raw_command_with_payload::<Success, _>(
                CommandType::Subscribe,
                serde_json::ser::to_string(events)?
            )?
            .success,
            "failed to subscribe to events"
        );
        Ok(EventIterator::new(self.0))
    }

    pub fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
        Ok(self.send_receive_raw_command(CommandType::GetOutputs)?)
    }

    pub fn get_tree(&mut self) -> Fallible<Node> {
        Ok(self.send_receive_raw_command(CommandType::GetTree)?)
    }

    pub fn get_marks(&mut self) -> Fallible<Vec<String>> {
        Ok(self.send_receive_raw_command(CommandType::GetMarks)?)
    }

    pub fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
        Ok(self.send_receive_raw_command(CommandType::GetBarConfig)?)
    }

    pub fn get_bar_config(&mut self, id: &str) -> Fallible<BarConfig> {
        Ok(self.send_receive_raw_command_with_payload(CommandType::GetBarConfig, id)?)
    }

    pub fn get_version(&mut self) -> Fallible<Version> {
        Ok(self.send_receive_raw_command(CommandType::GetVersion)?)
    }

    pub fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
        Ok(self.send_receive_raw_command(CommandType::GetBindingModes)?)
    }

    pub fn get_config(&mut self) -> Fallible<Config> {
        Ok(self.send_receive_raw_command(CommandType::GetConfig)?)
    }

    pub fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
        Ok(self
            .send_receive_raw_command_with_payload::<Success, _>(CommandType::SendTick, payload)?
            .success)
    }

    pub fn send_sync(&mut self) -> Fallible<bool> {
        Ok(self
            .send_receive_raw_command::<Success>(CommandType::Sync)?
            .success)
    }

    pub fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
        Ok(self.send_receive_raw_command(CommandType::GetInputs)?)
    }

    pub fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
        Ok(self.send_receive_raw_command(CommandType::GetSeats)?)
    }
}
