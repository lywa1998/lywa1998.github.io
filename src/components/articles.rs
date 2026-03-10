use dioxus::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

use crate::routes::sample_posts;

use super::post_card::Timeline;

#[component]
pub fn Articles() -> Element {
    let mut selected_year = use_signal(|| None::<String>);
    let mut selected_tag = use_signal(|| None::<String>);

    let all_posts = sample_posts();
    let total = all_posts.len();
    let mut years = BTreeSet::new();
    let mut tags = BTreeSet::new();

    for post in &all_posts {
        if let Some(year) = post.date.get(0..4) {
            years.insert(year.to_string());
        }
        for tag in &post.tags {
            tags.insert(tag.to_string());
        }
    }

    let year_filter = selected_year();
    let tag_filter = selected_tag();

    let filtered_posts: Vec<crate::routes::Post> = all_posts
        .into_iter()
        .filter(|post| {
            let year_ok = year_filter
                .as_ref()
                .is_none_or(|year| post.date.starts_with(year));
            let tag_ok = tag_filter
                .as_ref()
                .is_none_or(|tag| post.tags.iter().any(|post_tag| post_tag == tag));
            year_ok && tag_ok
        })
        .collect();

    let filtered_count = filtered_posts.len();
    let mut grouped_map: BTreeMap<String, Vec<crate::routes::Post>> = BTreeMap::new();
    for post in filtered_posts {
        let year = post.date.get(0..4).unwrap_or("Unknown").to_string();
        grouped_map.entry(year).or_default().push(post);
    }

    let mut grouped: Vec<(String, Vec<crate::routes::Post>)> = Vec::new();
    for (year, posts) in grouped_map.into_iter().rev() {
        if !posts.is_empty() {
            grouped.push((year, posts));
        }
    }
    let years: Vec<String> = years.into_iter().rev().collect();
    let has_filters = selected_year().is_some() || selected_tag().is_some();

    rsx! {
        section { class: "articles-page",
            h1 { class: "page-title", "Articles" }
            p { class: "page-subtitle", "记录技术实践、思考与项目总结。" }
            div { class: "articles-toolbar",
                p { class: "articles-toolbar__meta", "{filtered_count} / {total} posts" }
                div { class: "articles-toolbar__chips",
                    for year in years {
                        button {
                            class: if selected_year().as_ref() == Some(&year) { "chip chip--active" } else { "chip" },
                            r#type: "button",
                            onclick: {
                                let year = year.clone();
                                move |_| {
                                    if selected_year().as_ref() == Some(&year) {
                                        selected_year.set(None);
                                    } else {
                                        selected_year.set(Some(year.clone()));
                                    }
                                }
                            },
                            "{year}"
                        }
                    }
                    for tag in tags.into_iter().take(12) {
                        button {
                            class: if selected_tag().as_ref() == Some(&tag) { "chip chip--soft chip--active" } else { "chip chip--soft" },
                            r#type: "button",
                            onclick: {
                                let tag = tag.clone();
                                move |_| {
                                    if selected_tag().as_ref() == Some(&tag) {
                                        selected_tag.set(None);
                                    } else {
                                        selected_tag.set(Some(tag.clone()));
                                    }
                                }
                            },
                            "#{tag}"
                        }
                    }
                    if has_filters {
                        button {
                            class: "chip chip--clear",
                            r#type: "button",
                            onclick: move |_| {
                                selected_year.set(None);
                                selected_tag.set(None);
                            },
                            "Clear filters"
                        }
                    }
                }
            }

            if grouped.is_empty() {
                p { class: "articles-empty", "No posts match the current filters." }
            } else {
                for (year, year_posts) in grouped {
                    section { class: "articles-year",
                        h2 { class: "articles-year__title", "{year}" }
                        Timeline { posts: year_posts }
                    }
                }
            }
        }
    }
}
