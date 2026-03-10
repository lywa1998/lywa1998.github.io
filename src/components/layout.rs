use dioxus::prelude::*;
use super::nav::Nav;
use super::footer::Footer;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "layout",
            a {
                href: "#main-content",
                class: "skip-link",
                "Skip to main content"
            }
            Nav {}
            main {
                id: "main-content",
                class: "container layout-main",
                tabindex: "-1",
                Outlet::<crate::route_enum::Route> {}
            }
            Footer {}
        }
    }
}
