use super::*;

impl Command<DefaultBorder<()>> {
    pub fn normal(self) -> Command<Final> {
        self.push("normal").transmute()
    }

    pub fn none(self) -> Command<Final> {
        self.push("none").transmute()
    }

    pub fn pixel(self, px: usize) -> Command<Final> {
        self.push("pixel").push(px.to_string()).transmute()
    }
}
