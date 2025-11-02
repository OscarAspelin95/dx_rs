use crate::components::file_upload::AcceptFileTypes;
use crate::components::Button;
use crate::components::Separator;
use crate::components::ToastProvider;

use dioxus::html::FileData;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;
use dioxus_primitives::toast::ToastOptions;

use reqwest::multipart::{Form, Part};

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

/// The main issue here is the discrepancy between dx serve --desktop
/// and dx serve --web. In desktop mode, we have access to the file system,
/// file paths, etc. In web (browser) mode we only have access to browser specific
/// attributes:
/// * In desktop mode we only have to save the file path itself.
/// * For web mode, we need to store either the file contents (bytes) OR
///     we try to save the FileData object. Since we want to support
///     upload of multiple, large files - saving file contents is not feasible.
#[component]
pub fn FileInput() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;

    let handle_chosen_files = move |files: Vec<FileData>| async move {
        for file in files {
            // Add locally.
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

/// TODO - add progress spinner/loader.
#[component]
pub fn UploadButton() -> Element {
    let mut uploaded_files = consume_context::<UploadedFileContext>().uploaded_files;
    let toast_api = use_toast();

    let upload_files = move || async move {
        // Upload to server.
        let client = reqwest::Client::new();

        let files = uploaded_files.read().clone();
        // We might want to spawn separate background processes here...
        for file in files {
            // This is the main issue in desktop vs web. For web, we need to access
            // the actual file contents, not only the file name/path.

            // This works only in desktop mode.
            let part = Part::file(file.name).await.expect("bla");
            let payload = Form::new().part("file", part);

            // Actual upload.
            let response = client
                .post("http://localhost:8001/upload")
                .multipart(payload)
                .send()
                .await;

            match response {
                Ok(response) => {
                    info!("{:?}", response)
                }
                Err(e) => {
                    error!("{:?}", e)
                }
            }
        }

        // We need some condition to check if
        // All files were uploaded successfully.

        // Upload success.
        toast_api.success(
            "Successfully uploaded files".to_string(),
            ToastOptions::new()
                .duration(Duration::from_secs(3))
                .permanent(false),
        );

        // Remove locally chosen files.
        // NOTE - here, we should change to only
        // remove files that were successfully uploaded.
        // I.e., Keep files that failed to upload.
        uploaded_files.write().clear();
    };

    rsx! {
        if uploaded_files.len() > 0 {
            Button {
                id: "upload-button",
                onclick: move |_| async move {
                    upload_files().await;
                },
                "Upload"
            }
        }
    }
}

#[component]
pub fn UploadComponent() -> Element {
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
