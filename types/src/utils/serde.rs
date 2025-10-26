use serde::Deserialize;
use serde::de::{Deserializer, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

struct SkipNullValues<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for SkipNullValues<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a vec with optional elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(opt) = seq.next_element::<Option<T>>()? {
            if let Some(value) = opt {
                vec.push(value);
            }
        }
        Ok(vec)
    }
}

pub fn skip_null_values<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_seq(SkipNullValues(std::marker::PhantomData))
}
