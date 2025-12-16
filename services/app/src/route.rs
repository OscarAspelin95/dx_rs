use crate::views::MainNavBar;
use crate::views::{AuthCallback, Blog, Home, Login, Results, ToDo, Upload};
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    // Auth routes (no navbar layout)
    #[route("/login")]
    Login {},
    #[route("/auth/callback/google?:code")]
    AuthCallback { code: String },

    // Main app routes (with navbar layout)
    #[layout(MainNavBar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/upload")]
    Upload {},
    #[route("/results")]
    Results {},
    #[route("/todo")]
    ToDo {},
}
