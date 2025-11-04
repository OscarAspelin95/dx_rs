use crate::components::ToDoList;
use dioxus::prelude::*;

#[component]
pub fn ToDo() -> Element {
    rsx! {
        div { id: "todo-container", ToDoList {} }
    }
}
