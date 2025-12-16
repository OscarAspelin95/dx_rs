use crate::auth::ProtectedRoute;
use crate::components::UploadMain;
use dioxus::prelude::*;

#[component]
pub fn Upload() -> Element {
    rsx! {
        ProtectedRoute {
            div { id: "upload-container", UploadMain {} }
        }
    }
}
