use super::*;

impl Command<Unmark<()>> {
    pub fn all_identifier(self) -> Command<Final> {
        self.transmute()
    }

    pub fn identifier(self, identifier: impl AsRef<str>) -> Command<Final> {
        self.push(identifier).transmute()
    }
}
