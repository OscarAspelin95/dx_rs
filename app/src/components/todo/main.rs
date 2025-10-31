use std::time::Duration;

use super::types::{Label, Status, ToDoItem};
use crate::components::Button;
use crate::components::Checkbox;
use crate::components::Select;
use crate::components::SelectItemIndicator;
use crate::components::SelectList;
use crate::components::SelectOption;
use crate::components::SelectTrigger;
use crate::components::SelectValue;
use crate::components::Separator;
use crate::components::ToastProvider;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use reqwest;
use serde_json;
use strum::IntoEnumIterator;
use uuid;

#[component]
fn LabelIcon(label: Label) -> Element {
    let label_text: String = match label {
        Label::Api => "api".into(),
        Label::Database => "database".into(),
        Label::Dioxus => "genetics".into(),
        _ => "list".into(),
    };
    rsx! {
        span { id: "task-label-span", class: "material-symbols-outlined", "{label_text}" }
    }
}

#[component]
pub fn ToDoTaskList() -> Element {
    let toast_api = use_toast();

    let mut tasks: Signal<Vec<ToDoItem>> = consume_context::<Signal<Vec<ToDoItem>>>();

    let toggle_task = move |i: usize| async move {
        match tasks.get_mut(i) {
            Some(mut task) => {
                // Toggle locally.
                task.toggle();

                // Update database.
                let client = reqwest::Client::new();
                let response = client
                    .patch(format!("http://localhost:8001/toggle_task/{}", task.uuid))
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        info!("{:?}", response);
                    }
                    Err(e) => {
                        toast_api.error(
                            "Error".to_string(),
                            ToastOptions::new()
                                .description("Failed to update task in database.")
                                .duration(Duration::from_secs(3))
                                .permanent(false),
                        );
                        error!("{:?}", e);
                    }
                }
            }
            None => {
                warn!("Task does not exist for index {}", i);
            }
        }
    };

    let remove_task = move |i: usize| async move {
        // Return the locally removed task.
        let task = tasks.remove(i);

        spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .delete(format!("http://localhost:8001/remove_task/{}", task.uuid))
                .send()
                .await;

            match response {
                Ok(response) => {
                    info!("{:?}", response)
                }
                Err(e) => {
                    toast_api.error(
                        "Error".to_string(),
                        ToastOptions::new()
                            .description("Failed to update task in database.")
                            .duration(Duration::from_secs(3))
                            .permanent(false),
                    );
                    error!("{:?}", e)
                }
            }
        });
    };

    rsx! {
        div { id: "task-container",
            {tasks.iter().enumerate().map(|(i, task)| rsx! {
                div { id: "task-row",

                    div { id: "task-name-checkbox",
                        Checkbox {
                            id: "task-name-checkbox-checkbox",
                            on_checked_change: move |_| async move {
                                toggle_task(i).await;
                            },
                        }
                        span { class: if task.status == Status::Completed { "task-name checked" } else { "task-name" },
                            {format!("{}", &task.name.to_string())}
                        }
                    }

                    div { id: "label-with-remove-button-container",
                        LabelIcon { label: Label::Database }
                        Button {
                            id: "remove-task-button",
                            "data-style": "destructive",
                            onclick: move |_| async move {
                                remove_task(i).await;
                            },
                            "X"
                        }
                    }


                }
                Separator {}
            })}
        }
    }
}

#[component]
pub fn NewTask() -> Element {
    let toast_api = use_toast();
    let mut task_name = use_signal::<String>(|| String::new());
    let mut tasks = consume_context::<Signal<Vec<ToDoItem>>>();

    let create_new_task = move |mut task_name: Signal<String>| async move {
        if task_name.read().is_empty() {
            toast_api.error(
                "Error".to_string(),
                ToastOptions::new()
                    .description("Please enter a task name.")
                    .duration(Duration::from_secs(3))
                    .permanent(false),
            );
            return;
        }

        // New task instance.
        let new_task = ToDoItem {
            name: task_name.read().clone(),
            status: Status::Created,
            label: None,
            uuid: uuid::Uuid::now_v7().to_string(),
        };

        // Update locally
        tasks.write().push(new_task.clone());

        // Update database.
        spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .post("http://localhost:8001/add_task")
                .json(&new_task)
                .send()
                .await;
            match response {
                Ok(response) => info!("Success: {:?}", response),
                Err(e) => {
                    toast_api.error(
                        "Error".to_string(),
                        ToastOptions::new()
                            .description("Failed to update task in database.")
                            .duration(Duration::from_secs(3))
                            .permanent(false),
                    );
                    error!("Error: {:?}", e);
                    return;
                }
            }
        });

        // Clear on success.
        task_name.write().clear();
    };

    let labels = Label::iter().enumerate().map(|(i, label)| {
        rsx! {
            SelectOption::<Option<Label>> { index: i, value: label, text_value: "{label}",
                {label.to_string()}
                SelectItemIndicator {}
            }
        }
    });

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
            div { id: "label-dropdown",
                Select::<Option<Label>> { placeholder: "Label",
                    SelectTrigger { id: "label-dropdown-trigger", width: "110px", SelectValue {} }
                    SelectList { id: "label-dropdown-list", {labels} }
                }
            }
            Button {
                id: "create-new-task-button",
                onclick: move |_| async move {
                    create_new_task(task_name).await;
                },
                "Add"
            }
        }
        Separator { id: "todo-component-separator" }
    }
}

#[component]
pub fn RemoveAll() -> Element {
    let toast_api = use_toast();
    let mut tasks = consume_context::<Signal<Vec<ToDoItem>>>();

    let remove_all = move || async move {
        // Remove all locally.
        tasks.write().clear();

        // Remove all tasks from database.
        let client = reqwest::Client::new();
        let response = client
            .delete("http://localhost:8001/remove_all_tasks")
            .send()
            .await;

        match response {
            Ok(response) => {
                info!("{:?}", response);
            }
            Err(e) => {
                toast_api.error(
                    "Error".to_string(),
                    ToastOptions::new()
                        .description("Failed to remove tasks.")
                        .duration(Duration::from_secs(3))
                        .permanent(false),
                );
                error!("{:?}", e);
            }
        }
    };

    rsx! {
        Button {
            id: "",
            "data-style": "destructive",
            onclick: move |_| async move {
                remove_all().await;
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
        ToastProvider {
            div { id: "todo-upper",
                h1 { id: "todo-title", "ToDo List" }
                NewTask {}
                ToDoTaskList {}
                RemoveAll {}
            }
        }
    }
}
