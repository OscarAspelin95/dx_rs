use crate::components::file_upload::AcceptFileTypes;
use crate::components::Button;
use crate::components::Separator;
use crate::components::ToastProvider;

use dioxus::html::FileData;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;
use dioxus_primitives::toast::ToastOptions;

use reqwest;

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct UploadedFileContext {
    uploaded_files: Signal<Vec<UploadedFile>>,
}
// For now, we store the file name and its string contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UploadedFile {
    name: PathBuf,
}

#[component]
pub fn FileInput() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;

    let handle_chosen_files = move |files: Vec<FileData>| async move {
        for file in files {
            // Add.
            uploaded_files
                .write()
                .push(UploadedFile { name: file.path() });
        }
    };

    rsx! {
        div { id: "file-picker-container",
            input {
                id: "file-picker-input",
                r#type: "file",
                accept: AcceptFileTypes::Gz.to_str(),
                multiple: true,
                onchange: move |evt| async move { handle_chosen_files(evt.files()).await },
            }
            Button {
                r#type: "button",
                id: "clear-files-button",
                "data-style": "destructive",
                onclick: move |_| uploaded_files.write().clear(),
                disabled: if uploaded_files.len() > 0 { false } else { true },
                "Remove All"
            }
        }
    }
}

#[component]
pub fn DragDrop() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;

    let handle_chosen_files = move |files: Vec<FileData>| async move {
        for file in files {
            // Add.
            uploaded_files
                .write()
                .push(UploadedFile { name: file.path() });
        }
    };
    rsx! {
        div {
            id: "drag-drop-container",
            ondrop: move |evt| async move {
                evt.prevent_default();
                let dt = evt.data_transfer();
                handle_chosen_files(dt.files()).await
            },
            ondragover: move |evt| {
                evt.prevent_default();
            },
            "Or drop files here..."
        }
    }
}

#[component]
pub fn FileList() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;
    rsx! {
        // List of chosen files.
        div { id: "file-list-container",
            {
                uploaded_files
                    .iter()
                    .enumerate()
                    .map(|(i, f)| rsx! {
                        div { id: "file-list-row",
                            span { id: "chosen-file-name-span", {format!("{}", f.name.to_str().unwrap_or("Not found"))} }
                            Button {
                                id: "file-list-remove-row-button",
                                "data-style": "destructive",
                                onclick: move |_| {
                                    uploaded_files.write().remove(i);
                                },
                                "Remove"
                            }
                        }
                        Separator {}
                    })
            }
        }
    }
}

#[component]
pub fn UploadButton() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;
    let toast_api = use_toast();
    let mut server_response = use_signal(|| String::new());

    rsx! {
        if uploaded_files.len() > 0 {
            Button {
                id: "upload-button",
                onclick: move |_| async move {
                    toast_api
                        .success(
                            "Successfully uploaded files".to_string(),
                            ToastOptions::new().duration(Duration::from_secs(3)).permanent(false),
                        );
                    toast_api
                        .warning(
                            "File processing not implemented yet...".to_string(),
                            ToastOptions::new().duration(Duration::from_secs(3)).permanent(false),
                        );
                    uploaded_files.write().clear();
                    let response = process_files(uploaded_files.read().clone()).await.unwrap();
                    server_response.set(response);
                },
                "Upload"
            }
        }

        if !server_response.read().is_empty() {
            div { "Server response: {server_response}" }
        }
    }
}

#[component]
pub fn UploadComponent() -> Element {
    // We can actually fetch db stuff from here.
    use_resource(|| async {
        let body = reqwest::get("http://localhost:8001/create_person/1")
            .await
            .expect("Failed to fetch from endpoint.")
            .text()
            .await
            .expect("Failed to convert response to text.");

        info!("Response from db: {:?}", body);
    });

    // Enable modifying our uploaded files.
    let uploaded_files = use_signal(|| Vec::<UploadedFile>::new());

    // Provide this context to relevant child components.
    use_context_provider(|| UploadedFileContext {
        uploaded_files: uploaded_files,
    });

    rsx! {
        FileInput {}
        DragDrop {}
        FileList {}
        ToastProvider { UploadButton {} }
    }
}

#[server]
async fn process_files(files: Vec<UploadedFile>) -> Result<String, ServerFnError> {
    Ok("Processing message accepted".to_string())
}
