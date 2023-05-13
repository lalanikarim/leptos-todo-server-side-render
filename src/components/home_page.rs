use crate::components::new_todo::*;
use crate::components::todo_item::*;
use leptos::*;
use uuid::Uuid;

use crate::models::todo::Todo;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (todos, set_todos) = create_signal(cx, Vec::new());
    let (show_done, set_show_done) = create_signal(cx, true);

    set_todos.set(
        (1..=10)
            .map(|_| Todo {
                id: Some(Uuid::new_v4()),
                task: Uuid::new_v4().urn().to_string(),
                done: false,
            })
            .map(|t| {
                let (todo, set_todo) = create_signal(cx, t);
                (todo, set_todo)
            })
            .collect(),
    );

    let filtered_todos = move || {
        if show_done.get() {
            todos.get()
        } else {
            todos
                .get()
                .into_iter()
                .filter(|t| !t.0.get().done)
                .collect()
        }
    };

    provide_context(cx, set_todos);
    provide_context(cx, show_done);
    provide_context(cx, set_show_done);

    view! { cx,
        <div class="main">
            <h1 class="header">"Todos"</h1>
            <NewTodo />
            <div>
                <For
                    each=filtered_todos
                    key=|s:&(ReadSignal<Todo>,WriteSignal<Todo>)| {
                        let(todo,_) = s;
                        todo.get().id.unwrap()
                    }
                    view=move|cx,(todo,set_todo)| {
                        view!{
                            cx,
                            <TodoItem todo set_todo />
                        }
                    }
                />
            </div>
        </div>
    }
}
