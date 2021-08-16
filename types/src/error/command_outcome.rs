use crate::{CommandOutcome, Error, Fallible};

impl CommandOutcome {
    pub fn decode(command_outcome: CommandOutcome) -> Fallible<()> {
        if let Some(error) = command_outcome.error {
            Err(if error.parse_error {
                Error::CommandParse(error.message)
            } else {
                Error::CommandFailed(error.message)
            })
        } else {
            Ok(())
        }
    }
}
