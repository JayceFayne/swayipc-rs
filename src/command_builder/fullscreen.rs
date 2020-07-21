use super::builder::prelude::*;
use super::Fullscreen;

impl Builder<Fullscreen<()>> {
    pub fn enable(self) -> CommandBuilder {
        self.push("enable").transmute()
    }

    pub fn disable(self) -> CommandBuilder {
        self.push("disable").transmute()
    }

    pub fn toggle(self) -> CommandBuilder {
        self.push("toggle").transmute()
    }
}
