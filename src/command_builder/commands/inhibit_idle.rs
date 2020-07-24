use super::*;

impl Command<InhibitIdle<()>> {
    pub fn focus(self) -> Command<Final> {
        self.push("focus").transmute()
    }

    pub fn fullscreen(self) -> Command<Final> {
        self.push("fullscreen").transmute()
    }

    pub fn open(self) -> Command<Final> {
        self.push("open").transmute()
    }

    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }

    pub fn visible(self) -> Command<Final> {
        self.push("visible").transmute()
    }
}
