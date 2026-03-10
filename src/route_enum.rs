use dioxus::prelude::*;
use crate::components::layout::MainLayout;
use crate::components::home::Home;
use crate::components::articles::Articles;
use crate::components::about::About;
use crate::components::post::PostDetail;

/// Application routes
#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home,

        #[route("/articles")]
        Articles,

        #[route("/about")]
        About,

        #[route("/post/:slug")]
        PostDetail { slug: String },
}
