use crate::route_enum::Route;
use crate::routes::Post;
use dioxus::prelude::*;

/// PostCard 组件 - 单个博客卡片
#[derive(Props, Clone, PartialEq)]
pub struct PostCardProps {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
}

#[component]
pub fn PostCard(props: PostCardProps) -> Element {
    let to = Route::PostDetail {
        slug: props.slug.clone(),
    };
    let aria = format!("Read article {}", props.title);

    rsx! {
        Link {
            class: "timeline-card-link",
            to: to,
            aria_label: aria,
            article { class: "timeline-card",
                h2 { class: "timeline-card__title",
                    "{props.title}"
                }
                p { class: "timeline-card__summary",
                    "{props.summary}"
                }
                if !props.tags.is_empty() {
                    div { class: "timeline-card__tags",
                        for tag in &props.tags {
                            span { class: "tag tag--soft",
                                "{tag}"
                            }
                        }
                    }
                }
                span { class: "timeline-card__cta", "Read article →" }
            }
        }
    }
}

/// Timeline 组件 - 竖直时间线容器
#[component]
pub fn Timeline(posts: Vec<Post>) -> Element {
    let rows: Vec<(String, String, String, String, Vec<String>)> = posts
        .into_iter()
        .map(|post| (post.slug, post.title, post.date, post.summary, post.tags))
        .collect();

    rsx! {
        div { class: "timeline-list",
            for (slug, title, date, summary, tags) in rows {
                div { class: "timeline-row",
                    div { class: "timeline-rail",
                        time { class: "timeline-rail__date", datetime: date.as_str(), "{date}" }
                        div { class: "timeline-rail__dot" }
                    }
                    div { class: "timeline-content",
                        PostCard {
                            slug: slug,
                            title: title,
                            summary: summary,
                            tags: tags
                        }
                    }
                }
            }
        }
    }
}
