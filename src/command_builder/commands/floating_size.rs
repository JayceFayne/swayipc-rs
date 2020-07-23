use super::*;

impl Command<FloatingSize<()>> {
    pub fn width(self, width: isize) -> Command<FloatingSize<X<()>>> {
        self.push(width.to_string().as_str()).transmute()
    }
}

impl Command<FloatingSize<X<()>>> {
    pub fn height(self, height: isize) -> Command<Final> {
        self.push("x").push(height.to_string().as_str()).transmute()
    }
}
