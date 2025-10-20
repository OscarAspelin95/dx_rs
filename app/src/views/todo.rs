use crate::components::ToDoList;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/todo.css");

#[component]
pub fn ToDo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { id: "todo-container", ToDoList {} }
    }
}
