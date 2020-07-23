use super::*;

impl Command<FocusWrapping<()>> {
    pub fn yes(self) -> Command<Final> {
        self.push("yes").transmute()
    }

    pub fn no(self) -> Command<Final> {
        self.push("no").transmute()
    }

    pub fn force(self) -> Command<Final> {
        self.push("force").transmute()
    }

    pub fn workspace(self) -> Command<Final> {
        self.push("workspace").transmute()
    }
}
