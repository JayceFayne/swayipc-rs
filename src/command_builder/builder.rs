use std::fmt::Debug;
use std::marker::PhantomData;

pub struct Empty;
pub struct NoFilter;
pub struct Filter;

pub struct CommandBuilder<T> {
    inner: String,
    state: PhantomData<T>,
}

pub type EmptyCommandBuilder = CommandBuilder<Empty>;

impl EmptyCommandBuilder {
    pub fn new() -> Self {
        CommandBuilder {
            inner: String::new(),
            state: PhantomData,
        }
    }
}

impl CommandBuilder<NoFilter> {
    pub fn filter(mut self, filter: impl AsRef<str>) -> CommandBuilder<Filter> {
        self.inner.insert_str(0, filter.as_ref());
        self.transmute()
    }
}

impl<T> CommandBuilder<T> {
    pub(super) fn transmute<N>(self) -> CommandBuilder<N> {
        CommandBuilder {
            inner: self.inner,
            state: PhantomData,
        }
    }

    pub(super) fn push(mut self, string: &str) -> CommandBuilder<T> {
        self.inner.push(' ');
        self.inner.push_str(string);
        self
    }
}

impl<T> Debug for CommandBuilder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl AsRef<str> for CommandBuilder<Filter> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl AsRef<str> for CommandBuilder<NoFilter> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

pub(super) mod prelude {
    pub type Builder<T> = super::CommandBuilder<T>;
    pub type CommandBuilder = super::CommandBuilder<super::NoFilter>;
}
