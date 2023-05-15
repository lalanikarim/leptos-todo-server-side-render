use std::fmt::Display;
use surrealdb::sql::Id as SurrealId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, PartialOrd)]
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
