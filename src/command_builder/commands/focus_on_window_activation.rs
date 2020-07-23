use super::*;

impl Command<FocusOnWindowActivation<()>> {
    pub fn smart(self) -> Command<Final> {
        self.push("smart").transmute()
    }

    pub fn urgent(self) -> Command<Final> {
        self.push("urgent").transmute()
    }

    pub fn focus(self) -> Command<Final> {
        self.push("focus").transmute()
    }

    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }
}
