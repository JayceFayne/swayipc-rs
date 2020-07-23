use super::*;

impl Command<ShowMarks<()>> {
    pub fn yes(self) -> Command<Final> {
        self.push("yes").transmute()
    }

    pub fn no(self) -> Command<Final> {
        self.push("no").transmute()
    }
}
