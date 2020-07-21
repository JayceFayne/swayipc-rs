use std::marker::PhantomData;

mod builder;
mod fullscreen;
mod sway_move;

pub use builder::CommandBuilder;
use builder::EmptyCommandBuilder;

pub struct NoAuto<T>(PhantomData<T>);
pub struct By<T>(PhantomData<T>);
pub struct WinCon<T>(PhantomData<T>);
pub struct Workspace<T>(PhantomData<T>);
pub struct Output<T>(PhantomData<T>);
pub struct Direction<T>(PhantomData<T>);
pub struct To<T>(PhantomData<T>);
pub struct Move<T>(PhantomData<T>);
pub struct Fullscreen<T>(PhantomData<T>);

//TODO: macro

impl EmptyCommandBuilder {
    pub fn fullscreen(self) -> CommandBuilder<Fullscreen<()>> {
        self.push("fullscreen").transmute()
    }

    pub fn sway_move(self) -> CommandBuilder<Move<()>> {
        self.push("move").transmute()
    }
}
