use super::builder::prelude::*;
use super::{By, Direction, Move, NoAuto, Output, To, WinCon, Workspace};

impl Builder<Move<()>> {
    pub fn left(self) -> Builder<Move<Direction<()>>> {
        self.push("left").transmute()
    }

    pub fn right(self) -> Builder<Move<Direction<()>>> {
        self.push("right").transmute()
    }

    pub fn up(self) -> Builder<Move<Direction<()>>> {
        self.push("up").transmute()
    }

    pub fn down(self) -> Builder<Move<Direction<()>>> {
        self.push("down").transmute()
    }

    pub fn to(self) -> Builder<Move<To<()>>> {
        self.push("to").transmute()
    }

    pub fn window(self) -> Builder<Move<WinCon<()>>> {
        self.transmute()
    }

    pub fn container(self) -> Builder<Move<WinCon<()>>> {
        self.transmute()
    }
}

impl Builder<Move<Direction<()>>> {
    pub fn by(self, by: usize) -> CommandBuilder {
        self.push(by.to_string().as_ref()).transmute()
    }
}

impl Builder<Move<To<()>>> {
    pub fn mark(self, mark: impl AsRef<str>) -> CommandBuilder {
        self.push("mark").push(mark.as_ref()).transmute()
    }

    pub fn scratchpad(self) -> CommandBuilder {
        self.push("scratchpad").transmute()
    }
}

impl Builder<Move<WinCon<NoAuto<()>>>> {
    pub fn to(self) -> Builder<Move<WinCon<NoAuto<To<()>>>>> {
        self.transmute()
    }
}

impl Builder<Move<WinCon<NoAuto<To<()>>>>> {
    pub fn workspace(self) -> Builder<Move<WinCon<NoAuto<To<Workspace<()>>>>>> {
        self.push("workspace").transmute()
    }
}

impl Builder<Move<WinCon<NoAuto<To<Workspace<()>>>>>> {
    pub fn by(self) -> Builder<Move<WinCon<To<Workspace<To<By<()>>>>>>> {
        self.transmute()
    }

    pub fn current(self) -> CommandBuilder {
        self.push("current").transmute()
    }

    pub fn next(self) -> CommandBuilder {
        self.push("next").transmute()
    }

    pub fn prev(self) -> CommandBuilder {
        self.push("prev").transmute()
    }

    pub fn next_on_output(self) -> CommandBuilder {
        self.push("next_on_output").transmute()
    }

    pub fn prev_on_output(self) -> CommandBuilder {
        self.push("prev_on_output").transmute()
    }
}

impl Builder<Move<WinCon<To<Workspace<To<By<()>>>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> CommandBuilder {
        self.push(name.as_ref()).transmute()
    }

    pub fn number(self, id: usize) -> CommandBuilder {
        self.push("number")
            .push(id.to_string().as_str())
            .transmute()
    }
}

impl Builder<Move<WinCon<()>>> {
    pub fn to(self) -> Builder<Move<WinCon<To<()>>>> {
        self.transmute()
    }

    pub fn no_auto_back_and_forth(self) -> Builder<Move<WinCon<NoAuto<()>>>> {
        self.push("--no-auto-back-and-forth").transmute()
    }
}

impl Builder<Move<WinCon<To<()>>>> {
    pub fn workspace(self) -> Builder<Move<WinCon<NoAuto<To<Workspace<()>>>>>> {
        self.push("workspace").transmute()
    }

    pub fn output(self) -> Builder<Move<WinCon<To<Output<()>>>>> {
        self.push("output").transmute()
    }

    pub fn center(self) -> CommandBuilder {
        self.push("position").push("center").transmute()
    }

    pub fn mouse(self) -> CommandBuilder {
        self.push("position").push("mouse").transmute()
    }

    pub fn cursor(self) -> CommandBuilder {
        self.push("position").push("cursor").transmute()
    }

    pub fn pointer(self) -> CommandBuilder {
        self.push("position").push("pointer").transmute()
    }

    pub fn position(self, x: usize, y: usize) -> CommandBuilder {
        self.push("position")
            .push("x")
            .push(x.to_string().as_str())
            .push("y")
            .push(y.to_string().as_str())
            .transmute()
    }
}

impl Builder<Move<WinCon<To<Output<()>>>>> {
    pub fn left(self) -> CommandBuilder {
        self.push("left").transmute()
    }

    pub fn right(self) -> CommandBuilder {
        self.push("right").transmute()
    }

    pub fn up(self) -> CommandBuilder {
        self.push("up").transmute()
    }

    pub fn down(self) -> CommandBuilder {
        self.push("down").transmute()
    }

    pub fn by(self) -> Builder<Move<WinCon<To<Output<By<()>>>>>> {
        self.transmute()
    }
}

impl Builder<Move<WinCon<To<Output<By<()>>>>>> {
    pub fn name(self, name: impl AsRef<str>) -> CommandBuilder {
        self.push(name.as_ref()).transmute()
    }

    pub fn id(self, id: usize) -> CommandBuilder {
        self.push(id.to_string().as_str()).transmute()
    }
}
