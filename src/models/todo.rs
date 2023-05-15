use chrono::NaiveDateTime;
use leptos::*;
use serde::{Deserialize, Serialize};

use super::thing::Thing;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    pub id: Option<Thing>,
    pub task: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<NaiveDateTime>,
}

cfg_if::cfg_if! {
   if #[cfg(feature = "ssr")] {
        use crate::SurrealDbClient;
        use crate::SurrealThing;
        use surrealdb::opt::PatchOp;
        use chrono::{Utc};
    }
}

#[cfg(feature = "ssr")]
impl Todo {
    pub fn register() {
        _ = GetTodos::register();
        _ = AddTodo::register();
        _ = UpdateTodo::register();
    }
}

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    if let Some(db) = use_context::<SurrealDbClient>(cx) {
        let todos: Vec<Todo> = db.select("todos").await.unwrap();
        Ok(todos)
    } else {
        Ok(vec![])
    }
}

#[server(AddTodo, "/api")]
pub async fn add_todo(cx: Scope, task: String) -> Result<Option<Todo>, ServerFnError> {
    if let Some(db) = use_context::<SurrealDbClient>(cx) {
        let todo: Todo = db
            .create("todos")
            .content(Todo {
                id: None,
                task,
                done: false,
                created_at: Utc::now().naive_local(),
                completed_at: None,
            })
            .await
            .unwrap();
        Ok(Some(todo))
    } else {
        Ok(None)
    }
}

#[server(UpdateTodo, "/api")]
pub async fn update_todo(cx: Scope, id: Thing, done: bool) -> Result<Option<Todo>, ServerFnError> {
    if let Some(db) = use_context::<SurrealDbClient>(cx) {
        let completed_at = match done {
            false => None,
            true => Some(Utc::now().naive_local()),
        };
        let _: Result<serde_json::Value, surrealdb::Error> = db
            .update(("todos", Into::<SurrealThing>::into(id.clone())))
            .patch(PatchOp::replace("/done", done))
            .patch(match done {
                true => PatchOp::add("/completed_at", completed_at),
                false => PatchOp::remove("/completed_at"),
            })
            .await;
        let todo: Option<Todo> = db.select(id.as_pair()).await.unwrap();
        Ok(todo)
    } else {
        Ok(None)
    }
}
