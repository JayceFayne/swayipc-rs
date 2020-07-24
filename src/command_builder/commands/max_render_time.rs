use super::*;

impl Command<MouseWraping<()>> {
    pub fn off(self) -> Command<Final> {
        self.push("off").transmute()
    }

    pub fn msec(self, msec: usize) -> Command<Final> {
        self.push(msec.to_string()).transmute()
    }
}
