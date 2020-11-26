use crate::{CommandOutcome, Error, Fallible};

impl CommandOutcome {
    pub fn decode(command_outcome: CommandOutcome) -> Fallible<()> {
        if let Some(error) = command_outcome.error {
            if error.parse_error {
                Err(Error::CommandParse(error.message))
            } else {
                Err(Error::CommandFailed(error.message))
            }
        } else {
            Ok(())
        }
    }
}
