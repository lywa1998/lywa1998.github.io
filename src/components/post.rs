use dioxus::prelude::*;
use crate::routes::sample_posts;

#[derive(Props, Clone, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[component]
pub fn PostDetail(props: PostProps) -> Element {
    let posts = sample_posts();
    let post = posts.iter().find(|p| p.slug == props.slug);

    if let Some(post) = post {
        rsx! {
            article { class: "post-detail",
                h1 { class: "post-title-large",
                    "{post.title}"
                }
                time { class: "post-date", datetime: post.date.as_str(),
                    "{post.date}"
                }
                div { class: "post-content",
                    div { class: "tags",
                        for tag in &post.tags {
                            span { class: "tag",
                                "{tag}"
                            }
                        }
                    }
                    div { class: "content",
                        dangerous_inner_html: "{post.content}"
                    }
                }
            }
        }
    } else {
        rsx! {
            div { class: "not-found",
                h1 { "Post Not Found" }
                p { "The post you're looking for doesn't exist." }
            }
        }
    }
}
