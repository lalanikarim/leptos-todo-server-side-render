use crate::components::new_todo::*;
use crate::components::todo_item::*;
use crate::models::todo::get_todos;

use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (show_done, set_show_done) = create_signal(cx, true);

    let server_todos = create_resource(cx, || (), move |_| get_todos(cx));

    provide_context(cx, show_done);
    provide_context(cx, set_show_done);

    view! { cx,
        <div class="main">
            <h1 class="header">"Todos"</h1>
            <NewTodo />
            <div>
                <Suspense
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
                                        key=move|todo| todo.id.unwrap()
                                        view=move |cx,todo| {
                                            let (todo,set_todo) = create_signal(cx,todo);
                                            view! {
                                                cx,
                                                <TodoItem todo set_todo />
                                            }
                                        }
                                    />
                                </ErrorBoundary>
                            }
                        }
                    )}
                }
                </Suspense>
            </div>
        </div>
    }
}
