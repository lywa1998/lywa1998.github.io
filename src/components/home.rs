use dioxus::prelude::*;
use crate::route_enum::Route;
use crate::routes::sample_posts;

#[component]
pub fn Home() -> Element {
    let post_count = sample_posts().len();

    rsx! {
        section { class: "cv",
            div { class: "cv-hero",
                img {
                    class: "cv-avatar",
                    src: asset!("/assets/avatar.jpg"),
                    alt: "Dup avatar"
                }
                h1 { class: "page-title", "Dup" }
                p { class: "page-subtitle", "Rust / Fullstack Developer · Building content-first products" }
                p { class: "cv-intro",
                    "专注 Rust 与现代 Web 工程，擅长从 0 到 1 设计可维护的产品架构。"
                    "当前重点在 Dioxus 全栈应用、性能优化与开发者体验。"
                }
                div { class: "cv-actions",
                    Link {
                        class: "btn btn--primary",
                        to: Route::Articles {},
                        "查看文章"
                    }
                    a {
                        class: "btn btn--ghost",
                        href: "mailto:tencent.batboy157@passfwd.com",
                        "联系我"
                    }
                }
                p { class: "cv-meta", "{post_count} 篇技术文章已发布" }
            }

            div { class: "cv-grid",
                section { class: "cv-card",
                    h2 { "Profile" }
                    p { "长期关注可靠工程实践、前后端协同与开发体验设计。" }
                }

                section { class: "cv-card",
                    h2 { "Experience" }
                    ul { class: "cv-list",
                        li {
                            strong { "Senior Software Engineer" }
                            span { " · 2023 - Present" }
                            p { "负责全栈系统架构、核心模块开发与工程规范落地。" }
                        }
                        li {
                            strong { "Fullstack Engineer" }
                            span { " · 2020 - 2023" }
                            p { "主导多个中后台与内容平台建设，持续优化交付效率。" }
                        }
                    }
                }

                section { class: "cv-card",
                    h2 { "Skills" }
                    div { class: "skills",
                        span { class: "tag", "Rust" }
                        span { class: "tag", "Dioxus 0.7" }
                        span { class: "tag", "Axum" }
                        span { class: "tag", "PostgreSQL" }
                        span { class: "tag", "Docker" }
                        span { class: "tag", "CI/CD" }
                    }
                }

                section { class: "cv-card",
                    h2 { "Contact" }
                    p {
                        "Email: "
                        a { href: "mailto:tencent.batboy157@passfwd.com", "tencent.batboy157@passfwd.com" }
                    }
                    p {
                        "GitHub: "
                        a { href: "https://github.com/lywa1998", target: "_blank", rel: "noopener", "github.com/lywa1998" }
                    }
                }
            }
        }
    }
}
