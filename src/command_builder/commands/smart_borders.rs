use super::*;

impl Command<SmartBoarders<()>> {
    pub fn on(self) -> Command<Final> {
        self.push("on").transmute()
    }

    pub fn no_gaps(self) -> Command<Final> {
        self.push("no_gaps").transmute()
    }

    pub fn off(self) -> Command<Final> {
        self.push("off").transmute()
    }
}
