use crate::views::MainNavBar;
use crate::views::{Blog, Home, Results, Upload};
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(MainNavBar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/upload")]
    Upload {},
    #[route("/results")]
    Results {},
}
