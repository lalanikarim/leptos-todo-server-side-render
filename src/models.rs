use leptos::ReadSignal;
use leptos::WriteSignal;
use todo::Todo;
pub mod todo;
pub type TodoSignalPair = (ReadSignal<Todo>, WriteSignal<Todo>);
