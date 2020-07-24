use super::states::{Final, Valid};
use std::default::Default;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

//TODO: macro

pub struct Command<T = ()> {
    inner: String,
    state: PhantomData<T>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            state: PhantomData,
        }
    }

    pub fn filter(criteria: impl AsRef<str>) -> Self {
        Self::new().push(criteria)
    }

    pub fn for_window(criteria: impl AsRef<str>) -> Self {
        Self::new().push("for_window").push(criteria)
    }
}

impl Command<Final> {
    pub fn new_unchecked(inner: impl Into<String>) -> Self {
        Self {
            inner: inner.into(),
            state: PhantomData,
        }
    }
}

impl<T> Command<T> {
    pub(super) fn push(mut self, val: impl AsRef<str>) -> Self {
        if !self.inner.is_empty() {
            self.inner.push(' ');
        }
        self.push_without_whitespace(val)
    }

    pub(super) fn push_without_whitespace(mut self, val: impl AsRef<str>) -> Self {
        self.inner.push_str(val.as_ref());
        self
    }

    pub(super) fn transmute<N>(self) -> Command<N> {
        Command {
            inner: self.inner,
            state: PhantomData,
        }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Command<Final> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl AsRef<str> for Command<Final> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<Command<Final>> for String {
    fn from(command: Command<Final>) -> Self {
        command.inner
    }
}

impl<T> Display for Command<Valid<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<T> AsRef<str> for Command<Valid<T>> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<T> From<Command<Valid<T>>> for String {
    fn from(command: Command<Valid<T>>) -> Self {
        command.inner
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}
