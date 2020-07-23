use super::*;

impl Command<FocusFollowMouse<()>> {
    pub fn yes(self) -> Command<Final> {
        self.push("yes").transmute()
    }

    pub fn no(self) -> Command<Final> {
        self.push("no").transmute()
    }

    pub fn always(self) -> Command<Final> {
        self.push("always").transmute()
    }
}
