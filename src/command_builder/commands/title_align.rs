use super::*;

impl Command<TitleAlign<()>> {
    pub fn left(self) -> Command<Final> {
        self.push("left").transmute()
    }

    pub fn center(self) -> Command<Final> {
        self.push("center").transmute()
    }

    pub fn right(self) -> Command<Final> {
        self.push("right").transmute()
    }
}
