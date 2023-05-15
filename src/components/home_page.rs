use crate::components::new_todo::*;
use crate::components::todo_item::*;
use crate::models::todo::get_todos;
use crate::models::todo::AddTodo;
use crate::models::todo::UpdateTodo;

use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (show_done, set_show_done) = create_signal(cx, true);

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
    provide_context(cx, show_done);
    provide_context(cx, set_show_done);
    provide_context(cx, add_todo_action);
    provide_context(cx, update_todo_action);

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
                                        each=move || todos.clone()
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
