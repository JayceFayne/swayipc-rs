mod command_outcome;
mod command_type;
mod event;

use thiserror::Error as ThisError;

pub type Fallible<T> = Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("unexpected magic string, expected 'i3-ipc' but got '{}'", String::from_utf8_lossy(.0))]
    InvalidMagic([u8; 6]),
    #[error("did receive a reply with type '{0}' but send command with type '{1}'")]
    InvalidCommandType(u32, u32),
    #[error("received unimplemented event '{}' with type '{}'", String::from_utf8_lossy(.1), .0)]
    UnimplementedEvent(u32, Vec<u8>),
    #[error("failed to subscribe to events {0}")]
    SubscriptionFailed(String),
    #[error("command failed with '{0}'")]
    CommandFailed(String),
    #[error("command could not be parsed '{0}'")]
    CommandParse(String),
    #[error("could not find the socket for neither i3 nor sway")]
    SocketNotFound,
}
