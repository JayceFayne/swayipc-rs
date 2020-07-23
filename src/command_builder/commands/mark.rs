use super::*;

impl Command<Mark<()>> {
    pub fn add(self) -> Command<Mark<Add<()>>> {
        self.push("--add").transmute()
    }

    pub fn toggle(self) -> Command<Mark<Add<Replace<()>>>> {
        self.push("--toggle").transmute()
    }
}

impl Command<Mark<Add<()>>> {
    pub fn replace(self) -> Command<Mark<Add<Replace<()>>>> {
        self.push("--replace").transmute()
    }

    pub fn identifier(self, name: impl AsRef<str>) -> Command<Final> {
        self.push(name.as_ref()).transmute()
    }
}

impl Command<Mark<Add<Replace<()>>>> {
    pub fn identifier(self, name: impl AsRef<str>) -> Command<Final> {
        self.push(name.as_ref()).transmute()
    }
}
