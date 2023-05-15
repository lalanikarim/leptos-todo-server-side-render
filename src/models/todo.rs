use leptos::*;
use serde::{Deserialize, Serialize};

use super::thing::Thing;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    pub id: Option<Thing>,
    pub task: String,
    pub done: bool,
}

cfg_if::cfg_if! {
   if #[cfg(feature = "ssr")] {
        use crate::SurrealDbClient;
        use crate::SurrealThing;
    }
}

#[cfg(feature = "ssr")]
impl Todo {
    pub fn register() {
        _ = GetTodos::register();
        _ = AddTodo::register();
        _ = UpdateTodo::register();
    }
    pub fn change_done(self, done: bool) -> Self {
        let Todo { id, task, .. } = self;
        Self { id, task, done }
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
            })
            .await
            .unwrap();
        println!("Todo: {:?}", todo);
        Ok(Some(todo))
    } else {
        Ok(None)
    }
}

#[server(UpdateTodo, "/api")]
pub async fn update_todo(cx: Scope, todo: Todo, done: bool) -> Result<Option<Todo>, ServerFnError> {
    if let Some(db) = use_context::<SurrealDbClient>(cx) {
        let id: SurrealThing = todo.clone().id.unwrap().into();
        let todo: Todo = db
            .update(("todos", id))
            .content(todo.change_done(done))
            .await
            .unwrap();
        Ok(Some(todo))
    } else {
        Ok(None)
    }
}
