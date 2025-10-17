use crate::components::file_upload::UploadComponent;
use dioxus::prelude::*;

const UPLOAD_CSS: Asset = asset!("./style.css");

#[component]
pub fn UploadMain() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: UPLOAD_CSS }
        UploadComponent {}
    }
}
