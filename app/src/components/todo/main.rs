use crate::components::Button;
use crate::components::Checkbox;
use crate::components::Input;
use crate::components::Separator;
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "created")]
    Created,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ToDoItem {
    name: String,
    status: Status,
    id: usize,
}

impl ToDoItem {
    fn toggle(&mut self) {
        match self.status {
            Status::Completed => self.status = Status::Created,
            Status::Created => self.status = Status::Completed,
        }
    }
    fn new() -> Self {
        Self {
            name: "something".into(),
            status: Status::Created,
            id: 1,
        }
    }
}
#[component]
pub fn ToDoTaskList() -> Element {
    let mut tasks: Signal<Vec<ToDoItem>> = consume_context::<Signal<Vec<ToDoItem>>>();

    let toggle_task = move |i: usize| async move {
        match tasks.get_mut(i) {
            Some(mut task) => {
                task.toggle();
            }
            None => {
                warn!("Task does not exist for index {}", i);
            }
        }
    };

    rsx! {
        // List of chosen files.
        div { id: "task-container",
            {tasks.iter().enumerate().map(|(i, task)| rsx! {
                div { id: "task-row",

                    div { id: "task-name-checkbox",
                        Checkbox {
                            on_checked_change: move |_| async move {
                                toggle_task(i).await;
                            },
                        }
                        span { class: if task.status == Status::Completed { "task-name checked" } else { "task-name" },
                            {format!("{}", &task.name.to_string())}
                        }
                    }

                    Button {
                        id: "remove-task-button",
                        "data-style": "destructive",
                        onclick: move |_| {
                            tasks.write().remove(i);
                        },
                        "X"
                    }
                }
                Separator {}
            })}
        }
    }
}

#[component]
pub fn NewTask() -> Element {
    let mut task_name = use_signal::<String>(|| String::new());
    let mut tasks = consume_context::<Signal<Vec<ToDoItem>>>();
    rsx! {
        div { id: "new-task-container",
            input {
                id: "task-input",
                placeholder: "...",
                onchange: move |evt| {
                    let text: String = evt.parsed().unwrap();
                    task_name.set(text);
                },
            }
            Button {
                id: "",
                onclick: move |_| {
                    if !task_name.read().is_empty() {
                        tasks
                            .write()
                            .push(ToDoItem {
                                name: task_name.read().clone(),
                                status: Status::Created,
                                id: 0,
                            })
                    }
                },
                "Add"
            }
        }
    }
}

#[component]
pub fn RemoveAll() -> Element {
    let mut tasks = consume_context::<Signal<Vec<ToDoItem>>>();

    rsx! {
        Button {
            id: "",
            "data-style": "destructive",
            onclick: move |_| {
                tasks.write().clear();
            },
            "Clear All"
        }
    }
}
#[component]
pub fn ToDoList() -> Element {
    let tasks_signal: Signal<Vec<ToDoItem>> =
        use_signal::<Vec<ToDoItem>>(|| vec![ToDoItem::new(), ToDoItem::new()]);

    use_context_provider(|| tasks_signal);

    rsx! {
        div { id: "todo-upper",
            h1 { id: "todo-title", "ToDo List" }
            NewTask {}
            ToDoTaskList {}
            RemoveAll {}
        }
    }
}
