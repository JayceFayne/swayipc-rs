use super::states::Final;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

//TODO: macro

pub struct Command<T> {
    inner: String,
    state: PhantomData<T>,
}

impl Command<()> {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            state: PhantomData,
        }
    }

    pub fn filter(criteria: impl AsRef<str>) -> Self {
        Self::new().push(criteria.as_ref())
    }

    pub fn for_window(criteria: impl AsRef<str>) -> Self {
        Self::new().push("for_window").push(criteria.as_ref())
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
    pub(super) fn push(mut self, string: &str) -> Self {
        if !self.inner.is_empty() {
            self.inner.push(' ');
        }
        self.inner.push_str(string);
        self
    }

    pub(super) fn transmute<N>(self) -> Command<N> {
        Command {
            inner: self.inner,
            state: PhantomData,
        }
    }
}

impl<T> Debug for Command<T> {
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
