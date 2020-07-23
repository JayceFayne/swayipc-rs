use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

type Key<'a> = &'a str;
type Value<'a> = Cow<'a, str>;
type Map<'a> = HashMap<Key<'a>, Value<'a>>;

pub struct Final {
    inner: String,
}

pub struct Constructing<'a> {
    inner: Map<'a>,
}

pub struct Empty;

pub trait Insert<'a> {
    fn insert(self, key: Key<'a>, value: impl Into<Value<'a>>) -> Map<'a>;
}

impl<'a> Insert<'a> for Empty {
    fn insert(self, key: Key<'a>, value: impl Into<Value<'a>>) -> Map<'a> {
        let mut map = Map::new();
        map.insert(key, value.into());
        map
    }
}

impl<'a> Insert<'a> for Constructing<'a> {
    fn insert(mut self, key: Key<'a>, value: impl Into<Value<'a>>) -> Map<'a> {
        self.inner.insert(key, value.into());
        self.inner
    }
}

pub struct Filter<T> {
    state: T,
}

impl Filter<Empty> {
    pub fn new() -> Self {
        Self { state: Empty }
    }
}

impl Filter<Final> {
    pub fn new_unchecked(inner: impl Into<String>) -> Self {
        Self {
            state: Final {
                inner: inner.into(),
            },
        }
    }
}

impl<'a, T: Insert<'a>> Filter<T> {
    fn insert(self, key: Key<'a>, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        Filter {
            state: Constructing {
                inner: self.state.insert(key, value),
            },
        }
    }

    pub fn app_id(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("app_id", value)
    }

    pub fn class(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("class", value)
    }

    pub fn con_id(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("con_id", value)
    }

    pub fn con_mark(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("con_mark", value)
    }

    pub fn floating(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("floating", value)
    }

    pub fn id(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("id", value)
    }

    pub fn instance(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("instance", value)
    }

    pub fn pid(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("pid", value)
    }

    pub fn shell(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("shell", value)
    }

    pub fn tiling(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("tiling", value)
    }

    pub fn title(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("title", value)
    }

    pub fn urgent(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("urgent", value)
    }

    pub fn window_role(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("window_role", value)
    }

    pub fn window_type(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("window_type", value)
    }

    pub fn workspace(self, value: impl Into<Value<'a>>) -> Filter<Constructing<'a>> {
        self.insert("workspace", value)
    }
}

impl Filter<Constructing<'_>> {
    pub fn finalize(&self) -> Filter<Final> {
        let mut inner = String::new();
        inner.push('[');
        for (key, value) in self.state.inner.iter() {
            inner.push_str(key);
            inner.push('=');
            inner.push_str(value);
            inner.push(' ');
        }
        inner.pop();
        inner.push(']');
        Filter {
            state: Final { inner },
        }
    }
}

impl Debug for Filter<Constructing<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.state.inner, f)
    }
}

impl Display for Filter<Final> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.state.inner, f)
    }
}

impl AsRef<str> for Filter<Final> {
    fn as_ref(&self) -> &str {
        &self.state.inner
    }
}
