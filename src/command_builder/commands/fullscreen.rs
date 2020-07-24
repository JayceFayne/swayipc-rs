use super::*;

impl Command<Fullscreen<()>> {
    pub fn enable(self) -> Command<Valid<Fullscreen<()>>> {
        self.push("enable").transmute()
    }

    pub fn disable(self) -> Command<Valid<Fullscreen<()>>> {
        self.push("disable").transmute()
    }

    pub fn toggle(self) -> Command<Valid<Fullscreen<()>>> {
        self.push("toggle").transmute()
    }
}

impl Command<Valid<Fullscreen<()>>> {
    pub fn global(self) -> Command<Final> {
        self.push("global").transmute()
    }
}
