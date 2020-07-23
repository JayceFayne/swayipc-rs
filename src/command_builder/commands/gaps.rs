use super::*;

impl Command<Gaps<()>> {
    pub fn inner(self, px: usize) -> Command<Final> {
        self.push("inner").push(px.to_string().as_str()).transmute()
    }

    pub fn outer(self, px: usize) -> Command<Final> {
        self.push("outer").push(px.to_string().as_str()).transmute()
    }

    pub fn horizontal(self, px: usize) -> Command<Final> {
        self.push("horizontal")
            .push(px.to_string().as_str())
            .transmute()
    }

    pub fn vertical(self, px: usize) -> Command<Final> {
        self.push("vertical")
            .push(px.to_string().as_str())
            .transmute()
    }

    pub fn top(self, px: usize) -> Command<Final> {
        self.push("top").push(px.to_string().as_str()).transmute()
    }

    pub fn right(self, px: usize) -> Command<Final> {
        self.push("right").push(px.to_string().as_str()).transmute()
    }

    pub fn bottom(self, px: usize) -> Command<Final> {
        self.push("bottom")
            .push(px.to_string().as_str())
            .transmute()
    }

    pub fn left(self, px: usize) -> Command<Final> {
        self.push("left").push(px.to_string().as_str()).transmute()
    }
}
