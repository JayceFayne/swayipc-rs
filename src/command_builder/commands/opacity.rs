use super::*;

impl Command<Opacity<()>> {
    pub fn set(self, value: f32) -> Command<Final> {
        self.push("set")
            .push(value.to_string().as_str())
            .transmute()
    }

    pub fn plus(self, value: f32) -> Command<Final> {
        self.push("plus")
            .push(value.to_string().as_str())
            .transmute()
    }

    pub fn minus(self, value: f32) -> Command<Final> {
        self.push("minus")
            .push(value.to_string().as_str())
            .transmute()
    }
}
