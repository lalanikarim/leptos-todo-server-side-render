use std::fmt::Display;
use surrealdb::sql::Id as SurrealId;

use serde::{de::Visitor, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Hash)]
pub enum Id {
    String(String),
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Id::String(id) = &self;
        write!(f, "{id}")
    }
}

impl From<SurrealId> for Id {
    fn from(value: SurrealId) -> Self {
        if let SurrealId::String(id) = value {
            Self::String(id)
        } else {
            panic!("Invalid Id")
        }
    }
}

impl Into<SurrealId> for Id {
    fn into(self) -> SurrealId {
        match self {
            Id::String(id) => SurrealId::String(id),
        }
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (Id::String(left), Id::String(right)) = (&self, other);
        left.cmp(right)
    }
}

struct IdVisitor;
impl<'de> Visitor<'de> for IdVisitor {
    type Value = Id;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Surreal Id or string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Id::String(v.to_owned()))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        match map.next_entry::<String, String>() {
            Ok(Some((key, value))) => match key.as_ref() {
                "String" => Ok(Id::String(value)),
                _ => panic!("Unsupported SurrealDB Id encountered"),
            },
            Ok(None) => panic!("Unable to match"),
            Err(err) => Err(err),
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(IdVisitor)
    }
}
