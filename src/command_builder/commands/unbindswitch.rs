use super::*;

impl Command<Unbindswitch<()>> {
    pub fn switch(self) -> Command<Unbindswitch<With<()>>> {
        self.push("switch").transmute()
    }
}

impl Command<Unbindswitch<With<()>>> {
    pub fn state(self, state: impl AsRef<str>) -> Command<Final> {
        self.push_without_whitespace(":")
            .push_without_whitespace(state)
            .transmute()
    }
}
