use super::*;

impl Command<Layout<()>> {
    pub fn default(self) -> Command<Final> {
        self.push("default").transmute()
    }

    pub fn splith(self) -> Command<Final> {
        self.push("splith").transmute()
    }

    pub fn splitv(self) -> Command<Final> {
        self.push("splitv").transmute()
    }

    pub fn stacking(self) -> Command<Final> {
        self.push("stacking").transmute()
    }

    pub fn tabbed(self) -> Command<Final> {
        self.push("tabbed").transmute()
    }

    pub fn toggle(self) -> Command<Layout<X<()>>> {
        self.push("toggle").transmute()
    }
}

impl Command<Layout<X<()>>> {
    pub fn split(self) -> Command<Final> {
        self.push("split").transmute()
    }

    pub fn all(self) -> Command<Final> {
        self.push("all").transmute()
    }

    pub fn through(self, list: impl AsRef<str>) -> Command<Final> {
        self.push(list).transmute()
    }
}
