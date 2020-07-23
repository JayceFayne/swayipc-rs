use super::*;

//TODO: i3 compat

impl Command<HideEdgeBorders<()>> {
    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }

    pub fn vertical(self) -> Command<Final> {
        self.push("vertical").transmute()
    }

    pub fn horizontal(self) -> Command<Final> {
        self.push("horizontal").transmute()
    }

    pub fn both(self) -> Command<Final> {
        self.push("both").transmute()
    }

    pub fn smart(self) -> Command<Final> {
        self.push("smart").transmute()
    }

    pub fn smart_no_gaps(self) -> Command<Final> {
        self.push("smart_no_gaps").transmute()
    }
}
