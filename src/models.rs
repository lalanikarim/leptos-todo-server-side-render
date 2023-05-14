use leptos::ReadSignal;
use leptos::WriteSignal;
use todo::Todo;
pub mod id;
pub mod thing;
pub mod todo;
pub type TodoSignalPair = (ReadSignal<Todo>, WriteSignal<Todo>);
