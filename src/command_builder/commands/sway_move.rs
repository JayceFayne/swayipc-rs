use super::*;

impl Command<Move<()>> {
    pub fn left(self) -> Command<Move<Direction<()>>> {
        self.push("left").transmute()
    }

    pub fn right(self) -> Command<Move<Direction<()>>> {
        self.push("right").transmute()
    }

    pub fn up(self) -> Command<Move<Direction<()>>> {
        self.push("up").transmute()
    }

    pub fn down(self) -> Command<Move<Direction<()>>> {
        self.push("down").transmute()
    }

    pub fn absolute(self) -> Command<Move<Absolute<()>>> {
        self.push("absolute").transmute()
    }

    pub fn no_auto_back_and_forth(self) -> Command<Move<NoAuto<()>>> {
        self.push("--no-auto-back-and-forth").transmute()
    }

    pub fn position(self) -> Command<Move<Position<()>>> {
        self.push("position").transmute()
    }

    pub fn window(self) -> Command<Move<WinCon<()>>> {
        self.push("window").transmute()
    }

    pub fn container(self) -> Command<Move<WinCon<()>>> {
        self.push("container").transmute()
    }

    pub fn workspace(self) -> Command<Move<Workspace<()>>> {
        self.push("workspace").transmute()
    }
}

impl Command<Move<Workspace<()>>> {
    pub fn to(self) -> Command<Move<Workspace<To<()>>>> {
        self.push("to").transmute()
    }
}

impl Command<Move<Workspace<To<()>>>> {
    pub fn output(self) -> Command<Move<WinCon<To<Output<()>>>>> {
        self.push("output").transmute()
    }
}

impl Command<Move<Absolute<()>>> {
    pub fn position(self) -> Command<Move<Absolute<Position<()>>>> {
        self.push("position").transmute()
    }
}

impl Command<Move<Absolute<Position<()>>>> {
    pub fn pos_x(self, x: usize) -> Command<Move<Absolute<Position<X<With<()>>>>>> {
        self.push(x.to_string().as_str()).transmute()
    }

    pub fn center(self) -> Command<Final> {
        self.push("center").transmute()
    }
}

impl Command<Move<Absolute<Position<X<With<()>>>>>> {
    pub fn in_px(self) -> Command<Move<Absolute<Position<X<With<Y<()>>>>>>> {
        self.push("px").transmute()
    }

    pub fn in_ppt(self) -> Command<Move<Absolute<Position<X<With<Y<()>>>>>>> {
        self.push("ppt").transmute()
    }
}

impl Command<Move<Absolute<Position<X<With<Y<()>>>>>>> {
    pub fn pos_y(self, y: usize) -> Command<Move<Absolute<Position<X<With<Y<With<()>>>>>>>> {
        self.push(y.to_string().as_str()).transmute()
    }
}

impl Command<Move<Absolute<Position<X<With<Y<With<()>>>>>>>> {
    pub fn in_px(self) -> Command<Final> {
        self.push("px").transmute()
    }

    pub fn in_ppt(self) -> Command<Final> {
        self.push("ppt").transmute()
    }
}

impl Command<Move<Position<()>>> {
    pub fn pos_x(self, x: usize) -> Command<Move<Absolute<Position<X<With<()>>>>>> {
        self.push(x.to_string().as_str()).transmute()
    }

    pub fn center(self) -> Command<Final> {
        self.push("center").transmute()
    }

    pub fn mouse(self) -> Command<Final> {
        self.push("mouse").transmute()
    }

    pub fn cursor(self) -> Command<Final> {
        self.push("cursor").transmute()
    }

    pub fn pointer(self) -> Command<Final> {
        self.push("pointer").transmute()
    }
}

impl Command<Move<Direction<()>>> {
    //FIXME:with macro, workaround for not impl `AsRef<str>` for `Command<Move<Direction<()>>>`
    pub fn tiling(self) -> Command<Final> {
        self.transmute()
    }

    pub fn by(self, px: usize) -> Command<Final> {
        self.push(px.to_string().as_str()).push("px").transmute()
    }
}

impl Command<Move<NoAuto<()>>> {
    pub fn window(self) -> Command<Move<NoAuto<WinCon<()>>>> {
        self.push("window").transmute()
    }

    pub fn container(self) -> Command<Move<NoAuto<WinCon<()>>>> {
        self.push("container").transmute()
    }
}

impl Command<Move<NoAuto<WinCon<()>>>> {
    pub fn to(self) -> Command<Move<WinCon<NoAuto<To<()>>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<NoAuto<To<()>>>>> {
    pub fn workspace(self) -> Command<Move<WinCon<NoAuto<To<Workspace<()>>>>>> {
        self.push("workspace").transmute()
    }
}

impl Command<Move<WinCon<NoAuto<To<Workspace<()>>>>>> {
    pub fn with(self) -> Command<Move<WinCon<To<Workspace<To<With<()>>>>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To<Workspace<()>>>>> {
    pub fn with(self) -> Command<Move<WinCon<To<Workspace<To<With<()>>>>>>> {
        self.transmute()
    }

    pub fn current(self) -> Command<Final> {
        self.push("current").transmute()
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
}

impl Command<Move<WinCon<To<Workspace<To<With<()>>>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Final> {
        self.push(name.as_ref()).transmute()
    }

    pub fn number(self, id: usize) -> Command<Final> {
        self.push("number")
            .push(id.to_string().as_str())
            .transmute()
    }
}

impl Command<Move<WinCon<()>>> {
    pub fn to(self) -> Command<Move<WinCon<To<()>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To<()>>>> {
    pub fn mark(self, mark: impl AsRef<str>) -> Command<Final> {
        self.push("mark").push(mark.as_ref()).transmute()
    }

    pub fn scratchpad(self) -> Command<Final> {
        self.push("scratchpad").transmute()
    }

    pub fn workspace(self) -> Command<Move<WinCon<To<Workspace<()>>>>> {
        self.push("workspace").transmute()
    }

    pub fn output(self) -> Command<Move<WinCon<To<Output<()>>>>> {
        self.push("output").transmute()
    }
}

impl Command<Move<WinCon<To<Output<()>>>>> {
    pub fn left(self) -> Command<Final> {
        self.push("left").transmute()
    }

    pub fn right(self) -> Command<Final> {
        self.push("right").transmute()
    }

    pub fn up(self) -> Command<Final> {
        self.push("up").transmute()
    }

    pub fn down(self) -> Command<Final> {
        self.push("down").transmute()
    }

    pub fn current(self) -> Command<Move<WinCon<To<Output<()>>>>> {
        self.push("current").transmute()
    }

    pub fn with(self) -> Command<Move<WinCon<To<Output<With<()>>>>>> {
        self.transmute()
    }
}

impl Command<Move<WinCon<To<Output<With<()>>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> Command<Final> {
        self.push(name.as_ref()).transmute()
    }

    pub fn id(self, id: usize) -> Command<Final> {
        self.push(id.to_string().as_str()).transmute()
    }
}
