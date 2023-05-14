use serde::{Deserialize, Serialize};
use std::hash::Hash;
use surrealdb::sql::Thing as SurrealThing;

use super::id::Id;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Thing {
    pub tb: String,
    pub id: Id,
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

impl Hash for Thing {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Into::<String>::into(self.clone()).hash(state)
    }
}
