use super::*;

impl Command<Rename<()>> {
    pub fn current_workspace(self) -> Command<Rename<To<()>>> {
        self.push("workspace").transmute()
    }

    pub fn workspace(self, name: impl AsRef<str>) -> Command<Rename<To<()>>> {
        self.push("workspace").push(name.as_ref()).transmute()
    }
}

impl Command<Rename<To<()>>> {
    pub fn to(self, name: impl AsRef<str>) -> Command<Final> {
        self.push("to").push(name.as_ref()).transmute()
    }
}
