use crate::{CommandOutcome, Error::CommandFailed, Fallible};

impl CommandOutcome {
    pub fn decode(command_outcome: CommandOutcome) -> Fallible<()> {
        if let Some(error) = command_outcome.error {
            Err(CommandFailed(error))
        } else {
            Ok(())
        }
    }
}
