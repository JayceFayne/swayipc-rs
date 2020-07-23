use super::*;

impl Command<Unmark<()>> {
    pub fn all(self) -> Command<Final> {
        self.transmute()
    }

    pub fn with(self, identifier: impl AsRef<str>) -> Command<Final> {
        self.push(identifier.as_ref()).transmute()
    }
}
