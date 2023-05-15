use crate::components::new_todo::*;
use crate::components::todo_item::*;
use crate::models::todo::get_todos;
use crate::models::todo::AddTodo;
use crate::models::todo::Todo;
use crate::models::todo::UpdateTodo;
use crate::signals::ShowDone;

use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (show_done_signal, set_show_done_signal) = create_signal(cx, ShowDone(true));

    let show_done = move || {
        let ShowDone(show_done) = show_done_signal.get();
        show_done
    };
    let load_todos = create_action(cx, |&()| async { () });
    let add_todo_action = create_server_action::<AddTodo>(cx);
    let update_todo_action = create_server_action::<UpdateTodo>(cx);

    let server_todos = create_resource(
        cx,
        move || {
            (
                load_todos.version().get(),
                add_todo_action.version().get(),
                update_todo_action.version().get(),
            )
        },
        move |_| async move { get_todos(cx).await },
    );

    load_todos.dispatch(());
    provide_context(cx, show_done_signal);
    provide_context(cx, set_show_done_signal);
    provide_context(cx, add_todo_action);
    provide_context(cx, update_todo_action);

    let filtered_todos = move |todos: Vec<Todo>| {
        let mut todos = if show_done() {
            todos
        } else {
            todos.into_iter().filter(|todo| !todo.done).collect()
        };
        todos.sort_by_cached_key(|Todo { created_at, .. }: &Todo| (created_at.clone(),));
        todos
    };

    view! { cx,
        <div class="main">
            <h1 class="header">"Todos"</h1>
            <NewTodo />
            <div>
                <Transition
                    fallback=move || view! { cx, <div>"Loading ... "</div>}
                >
                {
                    move || {
                        server_todos.read(cx).map(|result| {

                            let todos = result.unwrap();
                            view! {
                                cx,
                                <ErrorBoundary
                                    fallback=|cx, errors| view! {
                                        cx,
                                        <div>"Error"</div>
                                        <ul>
                                        {
                                            move || errors.get()
                                            .into_iter()
                                            .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                            .collect::<Vec<_>>()
                                        }
                                        </ul>
                                    }
                                    >
                                    <For
                                        each=move || filtered_todos(todos.clone())
                                        key=move|todo| {
                                        todo.id.clone()
                                        }
                                        view=move |cx,todo| {
                                            let (todo,_) = create_signal(cx,todo);
                                            view! {
                                                cx,
                                                <TodoItem todo />
                                            }
                                        }
                                    />
                                </ErrorBoundary>
                            }
                        }
                    )}
                }
                </Transition>
            </div>
        </div>
    }
}
