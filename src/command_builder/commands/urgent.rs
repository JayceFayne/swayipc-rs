use super::*;

impl Command<Urgent<()>> {
    pub fn enable(self) -> Command<Final> {
        self.push("enable").transmute()
    }

    pub fn disable(self) -> Command<Final> {
        self.push("disable").transmute()
    }

    pub fn allow(self) -> Command<Final> {
        self.push("allow").transmute()
    }

    pub fn deny(self) -> Command<Final> {
        self.push("deny").transmute()
    }
}
