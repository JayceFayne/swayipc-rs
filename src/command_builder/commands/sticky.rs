use super::*;

impl Command<Sticky<()>> {
    pub fn enable(self) -> Command<Final> {
        self.push("enable").transmute()
    }

    pub fn disable(self) -> Command<Final> {
        self.push("disable").transmute()
    }

    pub fn toggle(self) -> Command<Final> {
        self.push("toggle").transmute()
    }
}
