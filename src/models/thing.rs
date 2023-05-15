use core::panic;
use serde::{de::Visitor, Deserialize, Serialize};
use std::hash::Hash;
use surrealdb::sql::Thing as SurrealThing;

use super::id::Id;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Thing {
    pub tb: String,
    pub id: Id,
}

impl Thing {
    pub fn as_pair(&self) -> (String, String) {
        let Thing { tb, id } = self;
        (tb.to_string(), id.to_string())
    }
}

impl Into<String> for Thing {
    fn into(self) -> String {
        let Thing { tb, id } = self;
        format!("{tb}:{id}")
    }
}

impl Into<surrealdb::sql::Thing> for Thing {
    fn into(self) -> SurrealThing {
        let Thing { tb, id } = self;
        SurrealThing { tb, id: id.into() }
    }
}

impl From<surrealdb::sql::Thing> for Thing {
    fn from(value: surrealdb::sql::Thing) -> Self {
        let SurrealThing { tb, id } = value;
        Self {
            tb,
            id: Id::from(id),
        }
    }
}

impl From<String> for Thing {
    fn from(value: String) -> Self {
        dbg!("From Thing: received: {value}");
        let splits: Vec<&str> = value.split(':').collect();
        if splits.len() != 2 {
            panic!("Invalid Thing format. Expected semicolon (:) separator. {value}")
        } else {
            let tb = splits[0].to_owned();
            let id = splits[1].to_owned();
            Thing {
                tb,
                id: Id::String(id),
            }
        }
    }
}

impl Serialize for Thing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tstr: String = Into::into(self.clone());
        serializer.serialize_str(&tstr)
    }
}

struct ThingVisitor;
impl<'de> Visitor<'de> for ThingVisitor {
    type Value = Thing;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string or surreal thing")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let thing = Thing::from(v.to_owned());
        Ok(thing)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tb = None;
        let mut id = None;

        let mut visitor_error: Option<A::Error> = None;
        for _ in 1..=2 {
            match map.next_key::<String>() {
                Ok(key) => {
                    if let Some(key) = key {
                        match key.as_ref() {
                            "tb" => tb = map.next_value().unwrap(),
                            "id" => id = map.next_value().unwrap(),
                            _ => {}
                        }
                    }
                }
                Err(err) => visitor_error = Some(err),
            }
        }
        if (tb.is_none() || id.is_none()) && visitor_error.is_none() {
            panic!("Invalid value provided: either tb or id or both are missing.")
        }
        if visitor_error.is_some() {
            Err(visitor_error.unwrap())
        } else {
            Ok(Thing {
                tb: tb.unwrap(),
                id: id.unwrap(),
            })
        }
    }
}

impl<'de> Deserialize<'de> for Thing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ThingVisitor)
    }
}
