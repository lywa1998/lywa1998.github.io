use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { class: "copyright",
                "© 2026 Dup. "
                span { "Powered by " }
                a { href: "https://dioxuslabs.com", target: "_blank", rel: "noopener", "Dioxus" }
                span { " & " }
                a { href: "https://github.com/sirice/hexo-theme-Chic", target: "_blank", rel: "noopener", "Chic Theme" }
            }
        }
    }
}
