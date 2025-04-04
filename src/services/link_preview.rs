use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use select::document::Document;
use select::predicate::{Attr, Name, Or, Predicate};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Default)]
pub struct UrlPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub favicon: Option<String>,
    pub url: String,
}

pub async fn get_url_preview(input_url: &str) -> Result<UrlPreview, Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(input_url)
        .map_err(|e| format!("Invalid URL '{}': {}", input_url, e))?;

    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err(format!("Unsupported URL scheme: {}", parsed_url.scheme()).into());
    }

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; RustBot/1.0)")
        .build()?;

    let response = client.get(input_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: HTTP {}", response.status()).into());
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    if !content_type.starts_with("text/html") {
        return Err(format!("Not an HTML page: {}", content_type).into());
    }

    let final_url = response.url().to_string();
    let body = response.text().await?;
    let document = Document::from(body.as_str());

    let mut meta_map = HashMap::new();
    for node in document.find(Name("meta")) {
        if let Some(name) = node.attr("name").or_else(|| node.attr("property")) {
            if let Some(content) = node.attr("content") {
                meta_map.insert(name.to_lowercase(), content.to_string());
            }
        }
    }

    let title = document
        .find(Name("title"))
        .next()
        .map(|n| n.text())
        .or_else(|| meta_map.get("og:title").cloned());

    let description = meta_map
        .get("description")
        .or_else(|| meta_map.get("og:description"))
        .cloned();

    let image = meta_map
        .get("og:image")
        .or_else(|| meta_map.get("twitter:image"))
        .cloned();

    let favicon = document
        .find(Or(Attr("rel", "icon"), Attr("rel", "shortcut icon")))
        .filter_map(|n| n.attr("href"))
        .map(|href| resolve_url(&final_url, href))
        .next();

    Ok(UrlPreview {
        title,
        description,
        image,
        favicon,
        url: final_url,
    })
}

fn resolve_url(base: &str, link: &str) -> String {
    Url::parse(base)
        .ok()
        .and_then(|base_url| base_url.join(link).ok())
        .map(|u| u.to_string())
        .unwrap_or_else(|| link.to_string())
}
