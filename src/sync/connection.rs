use super::common::receive_from_stream;
use super::socket::get_path;
use crate::command::{Command, CommandType};
use crate::reply::{
  BarConfig, CommandOutcome, Config, Input, Node, Output, Seat, Success, Version, Workspace,
};
use crate::{ensure, EventIterator, EventType, Fallible};
use serde::de::DeserializeOwned;
use serde_json::from_slice;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct Connection(UnixStream);

impl Connection {
  pub fn new() -> Fallible<Self> {
    Ok(Self(UnixStream::connect(get_path()?)?))
  }

  pub(crate) fn send_raw_command<'a>(&mut self, command: Command<'a>) -> Fallible<CommandType> {
    self.0.write_all(&command.encode()[..])?;
    Ok(command.command_type)
  }

  pub(crate) fn receive_raw_command<T: DeserializeOwned>(
    &mut self,
    command_type: CommandType,
  ) -> Fallible<T> {
    let (message_type, payload) = receive_from_stream(&mut self.0)?;
    ensure!(
      u32::from(command_type) == message_type,
      "did receive a message with another type than requested"
    );
    Ok(from_slice(&payload)?)
  }

  pub(crate) fn send_receive_raw_command<T: DeserializeOwned>(
    &mut self,
    command: Command,
  ) -> Fallible<T> {
    let command_type = self.send_raw_command(command)?;
    Ok(self.receive_raw_command(command_type)?)
  }

  pub fn run_command<T: AsRef<str>>(&mut self, payload: T) -> Fallible<Vec<CommandOutcome>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::RunCommand, payload))?)
  }

  pub fn get_workspaces(&mut self) -> Fallible<Vec<Workspace>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetWorkspaces))?)
  }

  pub fn subscribe(mut self, events: &[EventType]) -> Fallible<EventIterator> {
    ensure!(
      self
        .send_receive_raw_command::<Success>(command_new!(
          CommandType::Subscribe,
          serde_json::ser::to_string(events)?
        ))?
        .success,
      "failed to subscribe to events"
    );
    Ok(EventIterator::new(self.0))
  }

  pub fn get_outputs(&mut self) -> Fallible<Vec<Output>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetOutputs))?)
  }

  pub fn get_tree(&mut self) -> Fallible<Node> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetTree))?)
  }

  pub fn get_marks(&mut self) -> Fallible<Vec<String>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetMarks))?)
  }

  pub fn get_bar_ids(&mut self) -> Fallible<Vec<String>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetBarConfig))?)
  }

  pub fn get_bar_config(&mut self, id: &str) -> Fallible<BarConfig> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetBarConfig, id))?)
  }

  pub fn get_version(&mut self) -> Fallible<Version> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetVersion))?)
  }

  pub fn get_binding_modes(&mut self) -> Fallible<Vec<String>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetBindingModes))?)
  }

  pub fn get_config(&mut self) -> Fallible<Config> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetConfig))?)
  }

  pub fn send_tick<T: AsRef<str>>(&mut self, payload: T) -> Fallible<bool> {
    Ok(
      self
        .send_receive_raw_command::<Success>(command_new!(CommandType::SendTick, payload))?
        .success,
    )
  }

  pub fn send_sync(&mut self) -> Fallible<bool> {
    Ok(
      self
        .send_receive_raw_command::<Success>(command_new!(CommandType::Sync))?
        .success,
    )
  }

  pub fn get_inputs(&mut self) -> Fallible<Vec<Input>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetInputs))?)
  }

  pub fn get_seats(&mut self) -> Fallible<Vec<Seat>> {
    Ok(self.send_receive_raw_command(command_new!(CommandType::GetSeats))?)
  }
}
