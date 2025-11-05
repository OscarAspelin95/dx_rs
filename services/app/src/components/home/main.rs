use crate::components::Separator;
use crate::route::Route;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("./style.css");
const LOGO_CROP: Asset = asset!("assets/logo/logo_crop.png");

#[component]
pub fn HomeMain() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div { id: "home-main",

            div { id: "home-action",
                Link { id: "home-link", to: Route::Upload {},
                    span { id: "home-action-span", "Upload" }
                    span {
                        id: "home-action-logo",
                        class: "material-symbols-outlined",
                        "cloud_upload"
                    }
                }

            }

            div { id: "logo-with-separator",
                Separator { id: "logo-separator" }
                img { id: "logo-crop", src: LOGO_CROP }
                Separator { id: "logo-separator" }
            }


            div { id: "home-action",
                Link { id: "home-link", to: Route::Results {},

                    span { id: "home-action-span", "Results" }
                    span {
                        id: "home-action-logo",
                        class: "material-symbols-outlined",
                        "list"
                    }
                }
            }




        }
    }
}
