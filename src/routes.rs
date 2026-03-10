//! Blog data models

use std::collections::HashMap;

use include_dir::{include_dir, Dir, File};
use pulldown_cmark::{html, Options, Parser};

static POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

/// Post data model
#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// Project data model
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub link: String,
}

/// Site configuration
#[derive(Debug, Clone, PartialEq)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub avatar: String,
    pub social_links: Vec<(String, String)>,
}

/// Default site config - Chic theme style
pub fn default_config() -> SiteConfig {
    SiteConfig {
        title: "Dup".to_string(),
        description: "A minimalist blog".to_string(),
        avatar: "/assets/avatar.jpg".to_string(),
        social_links: vec![
            (
                "GitHub".to_string(),
                "https://github.com/lywa1998".to_string(),
            ),
            (
                "Twitter".to_string(),
                "https://twitter.com/yourname".to_string(),
            ),
        ],
    }
}

/// Load posts from markdown files in content/posts
pub fn sample_posts() -> Vec<Post> {
    let mut files = Vec::new();
    collect_markdown_files(&POSTS_DIR, &mut files);

    let mut posts: Vec<Post> = files
        .into_iter()
        .filter_map(markdown_file_to_post)
        .collect();

    // Date format is YYYY-MM-DD so lexicographic sort is safe.
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn collect_markdown_files<'a>(dir: &'a Dir<'a>, files: &mut Vec<&'a File<'a>>) {
    for file in dir.files() {
        if file.path().extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(file);
        }
    }

    for sub_dir in dir.dirs() {
        collect_markdown_files(sub_dir, files);
    }
}

fn markdown_file_to_post(file: &File<'_>) -> Option<Post> {
    let raw = file.contents_utf8()?;
    let (front_matter, body) = parse_front_matter(raw);

    let filename_slug = file
        .path()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("untitled")
        .to_string();

    let slug = front_matter
        .get("slug")
        .cloned()
        .unwrap_or_else(|| filename_slug.clone());

    let title = front_matter
        .get("title")
        .cloned()
        .unwrap_or_else(|| filename_slug.replace('-', " "));

    let date = front_matter
        .get("date")
        .cloned()
        .unwrap_or_else(|| "1970-01-01".to_string());

    let summary = front_matter
        .get("summary")
        .cloned()
        .unwrap_or_else(|| build_fallback_summary(body));

    let tags = front_matter
        .get("tags")
        .map(|v| parse_tags(v))
        .unwrap_or_default();

    let content = markdown_to_html(body);

    Some(Post {
        slug,
        title,
        date,
        summary,
        content,
        tags,
    })
}

fn parse_front_matter(source: &str) -> (HashMap<String, String>, &str) {
    let mut map = HashMap::new();

    let Some(rest) = source.strip_prefix("---\n") else {
        return (map, source);
    };

    let Some(split_idx) = rest.find("\n---\n") else {
        return (map, source);
    };

    let (raw_meta, body_with_sep) = rest.split_at(split_idx);
    let body = &body_with_sep[5..];

    for line in raw_meta.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_string();
            let value = strip_quotes(v.trim());
            map.insert(key, value);
        }
    }

    (map, body)
}

fn strip_quotes(value: &str) -> String {
    let trimmed = value.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn parse_tags(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    let inner = trimmed
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(trimmed);

    inner
        .split(',')
        .map(strip_quotes)
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect()
}

fn build_fallback_summary(markdown: &str) -> String {
    for line in markdown.lines() {
        let text = line.trim();
        if text.is_empty() || text.starts_with('#') || text.starts_with("```") {
            continue;
        }

        let cleaned = text
            .trim_start_matches(|c: char| {
                matches!(c, '-' | '*' | '>' | '[' | ']' | '(' | ')' | '`')
            })
            .trim();

        if !cleaned.is_empty() {
            return cleaned.chars().take(100).collect();
        }
    }

    "No summary provided.".to_string()
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Sample projects data
pub fn sample_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Project Alpha".to_string(),
            description: "A minimalist task manager".to_string(),
            link: "https://github.com/lywa1998/alpha".to_string(),
        },
        Project {
            name: "Project Beta".to_string(),
            description: "Real-time chat application".to_string(),
            link: "https://github.com/lywa1998/beta".to_string(),
        },
    ]
}
