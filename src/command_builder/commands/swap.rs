use super::*;

impl Command<Swap<()>> {
    pub fn with(self) -> Command<Swap<With<()>>> {
        self.push("container").push("with").transmute()
    }
}

impl Command<Swap<With<()>>> {
    pub fn id(self, id: usize) -> Command<Final> {
        self.push("id").push(id.to_string().as_str()).transmute()
    }

    pub fn con_id(self, con_id: usize) -> Command<Final> {
        self.push("con_id")
            .push(con_id.to_string().as_str())
            .transmute()
    }

    pub fn mark(self, mark: impl AsRef<str>) -> Command<Final> {
        self.push("mark").push(mark.as_ref()).transmute()
    }
}
