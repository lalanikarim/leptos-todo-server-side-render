use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<Uuid>,
    pub task: String,
    pub done: bool,
}

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    //use surrealdb::engine::remote::ws::Client;
    //let db = use_context::<surrealdb::Surreal<Client>>(cx).expect("db client should be present");

    //let result = db.query("select * from time::now()").await.unwrap();
    //log!("db result: {:?}", result);

    let todos = (1..=10)
        .map(|_| Todo {
            id: Some(Uuid::new_v4()),
            task: Uuid::new_v4().urn().to_string(),
            done: false,
        })
        .collect::<Vec<Todo>>();
    Ok(todos)
}
