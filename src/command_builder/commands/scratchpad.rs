use super::*;

impl Command<Scratchpad<()>> {
    pub fn show(self) -> Command<Final> {
        self.push("show").transmute()
    }
}
