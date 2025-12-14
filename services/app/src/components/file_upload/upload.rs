use crate::components::file_upload::AcceptFileTypes;
use crate::components::Button;

use crate::components::{
    Select, SelectItemIndicator, SelectList, SelectOption, SelectTrigger, SelectValue,
};

use crate::components::{HoverCard, HoverCardContent, HoverCardTrigger};

use crate::components::Separator;
use crate::components::ToastProvider;
use crate::components::{Input, Label};
use crate::components::{PopoverContent, PopoverRoot, PopoverTrigger};
use dioxus_primitives::ContentSide;
use shared::schema::schema::Pipeline;
use strum::IntoEnumIterator;

use dioxus::html::FileData;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;
use dioxus_primitives::toast::ToastOptions;

use reqwest::multipart::{Form, Part};

use std::time::Duration;

#[derive(Clone)]
struct UploadedFileContext {
    uploaded_files: Signal<Vec<UploadedFile>>,
}

#[derive(Clone)]
struct UploadedFile {
    file_data: FileData,
}

#[component]
pub fn FileInput() -> Element {
    let mut uploaded_files = use_context::<UploadedFileContext>().uploaded_files;

    let handle_chosen_files = move |files: Vec<FileData>| async move {
        for file in files {
            uploaded_files
                .write()
                .push(UploadedFile { file_data: file });
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
    let mut uploaded_files = use_context::<UploadedFileContext>().uploaded_files;

    let handle_chosen_files = move |files: Vec<FileData>| async move {
        for file in files {
            uploaded_files
                .write()
                .push(UploadedFile { file_data: file });
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
    let mut uploaded_files = use_context::<UploadedFileContext>().uploaded_files;
    rsx! {
        // List of chosen files.
        div { id: "file-list-container",
            {
                uploaded_files
                    .iter()
                    .enumerate()
                    .map(|(i, f)| rsx! {
                        div { id: "file-list-row",
                            span { id: "chosen-file-name-span", {f.file_data.name()} }
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
    let mut uploaded_files = use_context::<UploadedFileContext>().uploaded_files;
    let chosen_pipeline = use_context::<Signal<Option<Pipeline>>>();

    let toast_api = use_toast();

    let upload_files = move || async move {
        // Check before trying to upload.
        if chosen_pipeline.read().is_none() {
            toast_api.error(
                "No pipeline chosen.".to_string(),
                ToastOptions::new()
                    .duration(Duration::from_secs(3))
                    .permanent(false),
            );
            return;
        }

        let client = reqwest::Client::new();

        let files = uploaded_files.read().clone();
        for file in files {
            let file_bytes = match file.file_data.read_bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("Failed to read file bytes: {:?}", e);
                    continue;
                }
            };

            let file_name = file.file_data.name();
            let file_part = Part::bytes(file_bytes.to_vec())
                .file_name(file_name)
                .mime_str("application/octet-stream")
                .expect("Failed to set MIME type");
            let pipeline_part = Part::text(chosen_pipeline.read().as_ref().unwrap().to_string());

            let payload = Form::new()
                .part("file", file_part)
                .part("pipeline", pipeline_part);

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
pub fn UploadConfig() -> Element {
    let mut chosen_pipeline = use_context::<Signal<Option<Pipeline>>>();
    let mut open = use_signal(|| false);

    let labels = Pipeline::iter().enumerate().map(|(i, label)| {
        rsx! {
            SelectOption::<Option<Pipeline>> { index: i, value: label, text_value: "{label}",
                {label.to_string()}
                SelectItemIndicator {}
            }
        }
    });

    rsx! {

        div { id: "upload-config-container",
            div { id: "label-dropdown",
                Select::<Option::<Pipeline>> {
                    on_value_change: move |label: Option<Option<Pipeline>>| {
                        match label {
                            Some(optional_label) => {
                                info!("{:?}", optional_label);
                                chosen_pipeline.set(optional_label);
                            }
                            None => {}
                        }
                    },
                    placeholder: "Pipeline",
                    SelectTrigger { id: "label-dropdown-trigger", width: "200px", SelectValue {} }
                    SelectList { id: "label-dropdown-list", {labels} }
                }
            }
            div { id: "config-popover",
                PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                    PopoverTrigger { "Config" }
                    PopoverContent { id: "popover-content", gap: "0.25rem",
                        // Metadata.
                        div { id: "metadata",

                            div { id: "header-with-hover",
                                h3 { id: "header-h3", "Metadata" }
                                div { id: "",
                                    HoverCard {
                                        HoverCardTrigger {
                                            i { id: "hover-question-mark", "?" }
                                        }
                                        HoverCardContent { side: ContentSide::Bottom,
                                            div { padding: "1rem",
                                                "Global metadata to add for every sample that is uploaded in this batch."
                                            }
                                        }
                                    }
                                }
                            }



                            div { id: "metadata-input",
                                Label { html_for: "identifier", "Identifier" }
                                Input { id: "identifier", placeholder: "..." }
                            }
                            div { id: "metadata-input",
                                Label { html_for: "comment", "Comment" }
                                Input { id: "comment", placeholder: "..." }
                            }
                        }

                        Separator {}

                        div { id: "thresholds",
                            div { id: "header-with-hover",
                                h3 { id: "header-h3", "Thresholds" }
                                div { id: "",
                                    HoverCard {
                                        HoverCardTrigger {
                                            i { id: "hover-question-mark", "?" }
                                        }
                                        HoverCardContent { side: ContentSide::Bottom,
                                            div { padding: "1rem",
                                                "Thresholds to apply during fastq preprocessing."
                                            }
                                        }
                                    }
                                }
                            }
                            div { id: "threshold-input",
                                Label { html_for: "min-read-length", "Min read length" }
                                Input {
                                    id: "min-read-length",
                                    r#type: "number",
                                    step: 100,
                                    min: 0,
                                    placeholder: 200,
                                }
                            }
                            div { id: "threshold-input",
                                Label { html_for: "max-read-length", "Max read length" }
                                Input {
                                    id: "max-read-length",
                                    r#type: "number",
                                    step: 100,
                                    min: 0,
                                    placeholder: "∞",
                                }
                            }
                            div { id: "threshold-input",
                                Label { html_for: "min-phred", "Min phred" }
                                Input {
                                    id: "min-phred",
                                    r#type: "number",
                                    step: 1,
                                    min: 10,
                                    placeholder: 15,
                                    max: 60,
                                }
                            }
                        }
                        Button {
                            r#type: "button",
                            "data-style": "outline",
                            onclick: move |_| {
                                open.set(false);
                            },
                            "Save"
                        }

                    }
                }
            }
        }
    }
}

#[component]
pub fn UploadComponent() -> Element {
    // Enable modifying our uploaded files.
    let uploaded_files = use_signal(|| Vec::<UploadedFile>::new());
    let chosen_pipeline: Signal<Option<Pipeline>> = use_signal(|| None);

    // Provide this context to relevant child components.
    use_context_provider(|| UploadedFileContext {
        uploaded_files: uploaded_files,
    });

    //
    use_context_provider(|| chosen_pipeline);

    rsx! {
        UploadConfig {}
        FileInput {}
        DragDrop {}
        FileList {}
        ToastProvider { UploadButton {} }
    }
}
