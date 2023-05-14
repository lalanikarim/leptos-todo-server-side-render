use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

use crate::models::todo::{AddTodo, Todo};

#[component]
pub fn NewTodo(cx: Scope) -> impl IntoView {
    let add_todo_action = use_context::<Action<AddTodo, Result<Option<Todo>, ServerFnError>>>(cx)
        .expect("add_todo_action should exist");

    let show_done = use_context::<ReadSignal<bool>>(cx).expect("show_done should exist");
    let set_show_done = use_context::<WriteSignal<bool>>(cx).expect("set_show_done should exist");

    let button_text = move || {
        if show_done.get() {
            "Hide Done"
        } else {
            "Show Done"
        }
    };

    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input_element = input_element.get().expect("Input should be present");

        //let todo = Todo {
        //    id: Some(Uuid::new_v4()),
        //    task: input_element.value(),
        //    done: false,
        //};

        add_todo_action.dispatch(AddTodo {
            task: input_element.value(),
        });
        //let (todo, set_todo) = create_signal(cx, todo);
        //set_todos.update(|items| items.push((todo, set_todo)));
        input_element.set_value("");
    };
    view! {
        cx,

        <div class="add-new-todo">
            <h3>"Add new task"</h3>
            <div class="content">
                <form on:submit=on_submit>
                    <input class="input" node_ref=input_element type="text"/>
                    <button type="button" class="button button-clear" on:click=move |ev| {
                        ev.prevent_default();
                        let show = show_done.get();
                        set_show_done.set(!show);
                    }>
                    {button_text}
                    </button>
                </form>
            </div>
        </div>
    }
}
