use super::states::{Empty, Final};
use std::fmt::Display;
use std::marker::PhantomData;

pub trait EmptyFinal {}
impl EmptyFinal for Empty {}
impl EmptyFinal for Final {}

//TODO: document behavior on multiple fn call like `Filter::new().shell("a").shell("b")`

#[derive(Debug)]
pub struct Filter<T = Empty> {
    inner: String,
    state: PhantomData<T>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            state: PhantomData,
        }
    }
}

impl<T> Filter<T> {
    fn insert<N>(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Filter<N> {
        let mut inner = self.inner;
        if inner.is_empty() {
            inner.push('[');
        } else {
            inner.pop();
            inner.push(' ');
        }
        inner.push_str(key.as_ref());
        inner.push('=');
        inner.push_str(value.as_ref());
        inner.push(']');
        Filter {
            inner,
            state: PhantomData,
        }
    }
}

impl<T: EmptyFinal> Filter<T> {
    pub fn app_id(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("app_id", value)
    }

    pub fn class(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("class", value)
    }

    pub fn con_id(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("con_id", value)
    }

    pub fn con_mark(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("con_mark", value)
    }

    pub fn floating(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("floating", value)
    }

    pub fn id(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("id", value)
    }

    pub fn instance(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("instance", value)
    }

    pub fn pid(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("pid", value)
    }

    pub fn shell(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("shell", value)
    }

    pub fn tiling(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("tiling", value)
    }

    pub fn title(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("title", value)
    }

    pub fn urgent(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("urgent", value)
    }

    pub fn window_role(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("window_role", value)
    }

    pub fn window_type(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("window_type", value)
    }

    pub fn workspace(self, value: impl AsRef<str>) -> Filter<Final> {
        self.insert("workspace", value)
    }
}

impl Display for Filter<Final> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl AsRef<str> for Filter<Final> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
