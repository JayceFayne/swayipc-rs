use super::*;

impl Command<PopupDuringFullscreen<()>> {
    pub fn smart(self) -> Command<Final> {
        self.push("smart").transmute()
    }

    pub fn ignore(self) -> Command<Final> {
        self.push("ignore").transmute()
    }

    pub fn leave_fullscreen(self) -> Command<Final> {
        self.push("leave_fullscreen").transmute()
    }
}
