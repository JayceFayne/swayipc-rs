use super::*;

impl Command<Gaps<()>> {
    pub fn inner(self) -> Command<Gaps<Select<()>>> {
        self.push("inner").transmute()
    }

    pub fn outer(self) -> Command<Gaps<Select<()>>> {
        self.push("outer").transmute()
    }

    pub fn horizontal(self) -> Command<Gaps<Select<()>>> {
        self.push("horizontal").transmute()
    }

    pub fn vertical(self) -> Command<Gaps<Select<()>>> {
        self.push("vertical").transmute()
    }

    pub fn top(self) -> Command<Gaps<Select<()>>> {
        self.push("top").transmute()
    }

    pub fn right(self) -> Command<Gaps<Select<()>>> {
        self.push("right").transmute()
    }

    pub fn bottom(self) -> Command<Gaps<Select<()>>> {
        self.push("bottom").transmute()
    }

    pub fn left(self) -> Command<Gaps<Select<()>>> {
        self.push("left").transmute()
    }
}

impl Command<Gaps<Select<()>>> {
    pub fn all(self) -> Command<Gaps<Select<With<()>>>> {
        self.push("all").transmute()
    }

    pub fn current(self) -> Command<Gaps<Select<With<()>>>> {
        self.push("current").transmute()
    }
}

impl Command<Gaps<Select<With<()>>>> {
    pub fn set(self) -> Command<Gaps<Select<With<X<()>>>>> {
        self.push("set").transmute()
    }

    pub fn plus(self) -> Command<Gaps<Select<With<X<()>>>>> {
        self.push("plus").transmute()
    }

    pub fn minus(self) -> Command<Gaps<Select<With<X<()>>>>> {
        self.push("minus").transmute()
    }
}

impl Command<Gaps<Select<With<X<()>>>>> {
    pub fn amount(self, amount: usize) -> Command<Final> {
        self.push(amount.to_string()).transmute()
    }
}
