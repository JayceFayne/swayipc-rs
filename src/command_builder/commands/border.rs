use super::*;

impl Command<Border<()>> {
    pub fn csd(self) -> Command<Final> {
        self.push("csd").transmute()
    }

    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }

    pub fn toggle(self) -> Command<Final> {
        self.push("toggle").transmute()
    }

    pub fn normal(self) -> Command<Valid<Border<With<()>>>> {
        self.push("normal").transmute()
    }

    pub fn pixel(self) -> Command<Valid<Border<With<()>>>> {
        self.push("pixel").transmute()
    }
}

impl Command<Valid<Border<With<()>>>> {
    pub fn with(self, n: isize) -> Command<Final> {
        self.push(n.to_string()).transmute()
    }
}
