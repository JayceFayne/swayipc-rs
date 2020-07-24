use super::*;

impl Command<Workspace<()>> {
    pub fn no_auto_back_and_forth(self) -> Command<Workspace<With<()>>> {
        self.push("--no-auto-back-and-forth").transmute()
    }

    pub fn goto(self) -> Command<Workspace<With<()>>> {
        self.transmute()
    }

    pub fn next(self) -> Command<Final> {
        self.push("next").transmute()
    }

    pub fn prev(self) -> Command<Final> {
        self.push("prev").transmute()
    }

    pub fn next_on_output(self) -> Command<Final> {
        self.push("next_on_output").transmute()
    }

    pub fn prev_on_output(self) -> Command<Final> {
        self.push("prev_on_output").transmute()
    }

    pub fn back_and_forth(self) -> Command<Final> {
        self.push("back_and_forth").transmute()
    }

    pub fn name(self, name: impl AsRef<str>) -> Command<Workspace<X<()>>> {
        self.push(name).transmute()
    }
}

impl Command<Workspace<With<()>>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Final> {
        self.push(name).transmute()
    }

    pub fn number(self, number: isize) -> Command<Final> {
        self.push(number.to_string()).transmute()
    }
}

impl Command<Workspace<X<()>>> {
    pub fn gaps(self) -> Command<Workspace<X<Gaps<()>>>> {
        self.transmute()
    }

    pub fn output(self, output: impl AsRef<str>) -> Command<Final> {
        self.push(output).transmute()
    }
}

impl Command<Workspace<X<Gaps<()>>>> {
    pub fn inner(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("inner").transmute()
    }

    pub fn outer(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("outer").transmute()
    }

    pub fn horizontal(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("horizontal").transmute()
    }

    pub fn vertical(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("vertical").transmute()
    }

    pub fn top(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("top").transmute()
    }

    pub fn right(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("right").transmute()
    }

    pub fn bottom(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("bottom").transmute()
    }

    pub fn left(self) -> Command<Workspace<X<Gaps<By<()>>>>> {
        self.push("left").transmute()
    }
}

impl Command<Workspace<X<Gaps<By<()>>>>> {
    pub fn amount(self, amount: usize) -> Command<Final> {
        self.push(amount.to_string()).transmute()
    }
}
