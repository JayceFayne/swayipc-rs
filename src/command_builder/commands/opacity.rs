use super::*;

impl Command<Opacity<()>> {
    pub fn set(self) -> Command<Opacity<With<()>>> {
        self.push("set").transmute()
    }

    pub fn plus(self) -> Command<Opacity<With<()>>> {
        self.push("plus").transmute()
    }

    pub fn minus(self) -> Command<Opacity<With<()>>> {
        self.push("minus").transmute()
    }
}

impl Command<Opacity<With<()>>> {
    pub fn value(self, value: f32) -> Command<Final> {
        self.push(value.to_string()).transmute()
    }
}
