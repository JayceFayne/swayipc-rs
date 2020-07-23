use super::*;

impl Command<SmartGaps<()>> {
    pub fn on(self) -> Command<Final> {
        self.push("on").transmute()
    }

    pub fn off(self) -> Command<Final> {
        self.push("off").transmute()
    }
}
