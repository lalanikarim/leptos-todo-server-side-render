use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

type TodoSignalPair = (ReadSignal<Todo>, WriteSignal<Todo>);
/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
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
        <div>
        <h1>"Todos"</h1>
        <NewTodo />
        <div class="todo-list">
        <For
            each=filtered_todos
            key=|s:&(ReadSignal<Todo>,WriteSignal<Todo>)| {
                let(todo,_) = s;
                todo.get().id.unwrap()
            }
            view=move|cx,(todo,set_todo)| {
                view!{
                    cx,
                    <div>
                    <TodoItem todo set_todo />
                    </div>
                }
            }

        />
        </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Todo {
    id: Option<Uuid>,
    task: String,
    done: bool,
}

#[component]
fn TodoItem(cx: Scope, todo: ReadSignal<Todo>, set_todo: WriteSignal<Todo>) -> impl IntoView {
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
            <div class="controls">
                <button class="action" on:click=on_click>{button_text}</button>
            </div>
        </div>
    }
}

#[component]
fn NewTodo(cx: Scope) -> impl IntoView {
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

        log!("Create: {:?}", todo);
        let (todo, set_todo) = create_signal(cx, todo);
        set_todos.update(|items| items.push((todo, set_todo)));
        input_element.set_value("");
    };
    view! {
        cx,

        <div class="new-todo">
            <div class="title">"Add new task"</div>
            <form on:submit=on_submit>
                <input class="input" node_ref=input_element type="text"/>
                <button class="action" type="submit">"Add"</button>
            </form>
            <div>"Show done:"</div>
            <input type="checkbox" on:change=move |ev| {
                let show = event_target_checked(&ev);
                log!("Status: {}",show);
                set_show_done.set(show);
            } />
        </div>
    }
}
