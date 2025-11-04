use crate::components::UploadMain;
use dioxus::prelude::*;

#[component]
pub fn Upload() -> Element {
    rsx! {

        div { id: "upload-container", UploadMain {} }
    }
}
