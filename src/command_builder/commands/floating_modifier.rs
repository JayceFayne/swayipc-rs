use super::*;

impl Command<FloatingModifier<()>> {
    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }

    pub fn with(self, modifier: impl AsRef<str>) -> Command<FloatingModifier<With<()>>> {
        self.push(modifier).transmute()
    }
}

impl Command<FloatingModifier<With<()>>> {
    pub fn normal(self) -> Command<Final> {
        self.push("normal").transmute()
    }

    pub fn inverse(self) -> Command<Final> {
        self.push("inverse").transmute()
    }
}
