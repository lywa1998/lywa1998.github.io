use dioxus::prelude::*;
use crate::routes::sample_projects;

#[component]
pub fn About() -> Element {
    let projects = sample_projects();

    rsx! {
        section { class: "about",
            h1 { class: "page-title", "About" }

            div { class: "about-content",
                div { class: "about-section",
                    h2 { "About Me" }
                    p {
                        "Hello! I'm a developer passionate about building elegant software. "
                        "This blog is where I share my thoughts on programming, design, and life."
                    }
                }

                div { class: "about-section",
                    h2 { "Projects" }
                    div { class: "project-list",
                        for project in projects {
                            div { class: "project-item",
                                h3 { class: "project-name",
                                    a { href: project.link.as_str(), target: "_blank",
                                        "{project.name}"
                                    }
                                }
                                p { class: "project-description",
                                    "{project.description}"
                                }
                            }
                        }
                    }
                }

                div { class: "about-section",
                    h2 { "Contact" }
                    p {
                        "Feel free to reach out: "
                        a { href: "mailto:tencent.batboy157@passfwd.com", "tencent.batboy157@passfwd.com" }
                    }
                }
            }
        }
    }
}
