use super::*;

impl Command<Split<()>> {
    pub fn vertical(self) -> Command<Final> {
        self.push("v").transmute()
    }

    pub fn horizontal(self) -> Command<Final> {
        self.push("h").transmute()
    }

    pub fn toggle(self) -> Command<Final> {
        self.push("t").transmute()
    }
}
