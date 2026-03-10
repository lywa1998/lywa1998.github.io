use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            h1 { class: "site-title",
                Link {
                    to: crate::route_enum::Route::Home {},
                    "Dup"
                }
            }
            p { class: "site-description",
                "Personal CV & technical articles powered by Dioxus"
            }
        }
    }
}
