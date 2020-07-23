use super::*;

impl Command<MouseWraping<()>> {
    pub fn output(self) -> Command<Final> {
        self.push("output").transmute()
    }

    pub fn container(self) -> Command<Final> {
        self.push("container").transmute()
    }

    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }
}
