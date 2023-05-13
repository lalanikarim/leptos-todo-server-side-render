use leptos::*;

use crate::models::todo::Todo;

#[component]
pub fn TodoItem(cx: Scope, todo: ReadSignal<Todo>, set_todo: WriteSignal<Todo>) -> impl IntoView {
    let button_text = move || {
        if todo.get().done {
            "undo"
        } else {
            "done"
        }
    };
    let on_click = move |_| {
        let todo = todo.get();
        let Todo { id, task, done } = todo;
        let new_todo = Todo {
            id,
            task,
            done: !done,
        };
        set_todo.set(new_todo);
    };
    view! {
        cx,
        <div class="todo-item">
            <label class="label" class:done=move||{todo.get().done}>{todo.get().task}</label>
            <button class="action" on:click=on_click>{button_text}</button>
        </div>
    }
}
