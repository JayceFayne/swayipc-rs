use super::*;

impl Command<TitlebarPadding<()>> {
    pub fn horizontal(self, px: usize) -> Command<TitlebarPadding<With<()>>> {
        self.push(px.to_string()).transmute()
    }
}

impl Command<TitlebarPadding<With<()>>> {
    pub fn vertical(self, px: usize) -> Command<Final> {
        self.push(px.to_string()).transmute()
    }
}
