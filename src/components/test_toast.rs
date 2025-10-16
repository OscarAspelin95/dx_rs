use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

use crate::components::Button;
use crate::components::ToastProvider;

#[component]
pub fn TestToast() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn DnaIcon() -> Element {
    rsx! {
        span { id: "dna_icon", class: "material-symbols-outlined", "genetics" }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast_api = use_toast();

    rsx! {
        Button {
            r#type: "button",
            id: "toast-button",
            "data-style": "outline",
            onclick: move |_| {
                toast_api
                    .success(
                        "Success".to_string(),
                        ToastOptions::new()
                            .description("Action was successful!")
                            .duration(Duration::from_secs(10))
                            .permanent(false),
                    );
            },
            "Click for toast!"
        }

        DnaIcon {}
    }
}
