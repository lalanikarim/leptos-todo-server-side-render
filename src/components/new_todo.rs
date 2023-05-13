use crate::models::todo::Todo;
use crate::models::TodoSignalPair;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn NewTodo(cx: Scope) -> impl IntoView {
    let set_todos =
        use_context::<WriteSignal<Vec<TodoSignalPair>>>(cx).expect("set_todos should exist");

    let set_show_done = use_context::<WriteSignal<bool>>(cx).expect("set_show_done should exist");

    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input_element = input_element.get().expect("Input should be present");

        let todo = Todo {
            id: Some(Uuid::new_v4()),
            task: input_element.value(),
            done: false,
        };

        let (todo, set_todo) = create_signal(cx, todo);
        set_todos.update(|items| items.push((todo, set_todo)));
        input_element.set_value("");
    };
    view! {
        cx,

        <div class="add-new-todo">
            <h3>"Add new task"</h3>
            <div class="row">
                <form on:submit=on_submit class="column-50">
                    <input class="input" node_ref=input_element type="text"/>
                    <button class="action" type="submit">"Add"</button>
                </form>
                <label>"Show done:"
                    <input type="checkbox" on:change=move |ev| {
                        let show = event_target_checked(&ev);
                        log!("Status: {}",show);
                        set_show_done.set(show);
                    } />
                </label>
            </div>
        </div>
    }
}
