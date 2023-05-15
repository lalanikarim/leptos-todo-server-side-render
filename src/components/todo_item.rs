use leptos::*;

use crate::models::todo::{Todo, UpdateTodo};

#[component]
pub fn TodoItem(cx: Scope, todo: ReadSignal<Todo>) -> impl IntoView {
    let update_todo_action =
        use_context::<Action<UpdateTodo, Result<Option<Todo>, ServerFnError>>>(cx)
            .expect("update_todo_action should exist");
    let button_text = move || {
        if todo.get().done {
            "undo"
        } else {
            "done"
        }
    };
    let done = move || todo.get().done;
    let task = move || todo.get().task;
    let on_click = move |_| {
        let todo = todo.get();
        let done = todo.done;
        update_todo_action.dispatch(UpdateTodo { todo, done: !done });
    };
    view! {
        cx,
        <div class="todo-item">
            <div class="task" class:done=done>{task}</div>
            <button class="button" class=("button-outline",done) on:click=on_click>{button_text}</button>
        </div>
    }
}
