use crate::components::Button;
use crate::components::Checkbox;
use crate::components::Separator;
use dioxus::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

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
    uuid: String,
}

impl ToDoItem {
    /// Can we run the db api query here?
    fn toggle(&mut self) {
        match self.status {
            Status::Completed => self.status = Status::Created,
            Status::Created => self.status = Status::Completed,
        }
    }
}
#[component]
pub fn ToDoTaskList() -> Element {
    let mut tasks: Signal<Vec<ToDoItem>> = consume_context::<Signal<Vec<ToDoItem>>>();

    let toggle_task = move |i: usize| async move {
        match tasks.get_mut(i) {
            Some(mut task) => {
                // Otherwise, we need to do it here.
                task.toggle();
            }
            None => {
                warn!("Task does not exist for index {}", i);
            }
        }
    };

    rsx! {
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
                        let new_task = ToDoItem {
                            name: task_name.read().clone(),
                            status: Status::Created,
                            uuid: "some_uuid".into(),
                        };
                        tasks.write().push(new_task.clone());
                        spawn(async move {
                            let client = reqwest::Client::new();
                            let response = client
                                .post("http://localhost:8001/add_task")
                                .json(&new_task)
                                .send()
                                .await;
                            match response {
                                Ok(response) => info!("Success: {:?}", response),
                                Err(e) => error!("Error: {:?}", e),
                            }
                        });
                        task_name.write().clear();
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
    // Here, we mock data. Instead, we'd like to fetch from database...
    let mut tasks_signal: Signal<Vec<ToDoItem>> = use_signal::<Vec<ToDoItem>>(|| vec![]);
    use_context_provider(|| tasks_signal);

    // We avoid db fetching on every render by use_effect with an empty dependency array.
    use_effect(move || {
        spawn(async move {
            let response = reqwest::get("http://localhost:8001/tasks").await;

            match response {
                Ok(response) => {
                    info!("Reponse OK: {:?}", response);

                    match serde_json::from_slice::<Vec<ToDoItem>>(
                        &response
                            .bytes()
                            .await
                            .expect("Failed to convert reponse to bytes."),
                    ) {
                        Ok(tasks) => {
                            info!("{:?}", tasks);
                            tasks_signal.set(tasks);
                        }
                        Err(e) => {
                            error!("Failed to serialize reponse bytes: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to fetch data. {:?}", e);
                }
            }
        });
    });

    rsx! {
        div { id: "todo-upper",
            h1 { id: "todo-title", "ToDo List" }
            NewTask {}
            ToDoTaskList {}
            RemoveAll {}
        }
    }
}
