use std::collections::{HashSet};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use reqwest::blocking::Client;
use threadpool::ThreadPool;
use scraper::{Html, Selector};

pub fn crawl(
    client: &Client,
    url: &str,
    output_dir: &PathBuf,
    depth: usize,
    visited: &Arc<Mutex<HashSet<String>>>,
    pool: &ThreadPool,
    parent_name: Option<String>,
) {
    if depth == 0 {
        return;
    }
    let url_string = url.to_string();
    {
        let mut visited = visited.lock().unwrap();
        if visited.contains(&url_string) {
            return;
        }
        visited.insert(url_string.clone());
    }
    let resp = client.get(url).send();
    if let Ok(resp) = resp {
        if let Ok(body) = resp.text() {
            // Save HTML
            let page_name = url_to_filename(url);
            let save_dir = if let Some(parent) = &parent_name {
                let mut d = output_dir.clone();
                d.push(parent);
                std::fs::create_dir_all(&d).ok();
                d
            } else {
                output_dir.clone()
            };
            let mut save_path = save_dir.clone();
            save_path.push(format!("{}.html", page_name));
            match std::fs::write(&save_path, &body) {
                Ok(_) => println!("Saved: {}", save_path.display()),
                Err(e) => eprintln!("Failed to save {}: {}", save_path.display(), e),
            }

            // Parse links
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a[href]").unwrap();
            for element in document.select(&selector) {
                if let Some(link) = element.value().attr("href") {
                    if let Ok(next_url) = url::Url::parse(link).or_else(|_| url::Url::parse(&format!("{}{}", url, link))) {
                        let next_url_str = next_url.as_str().to_string();
                        let visited = Arc::clone(visited);
                        let output_dir = output_dir.clone();
                        let client = client.clone();
                        let pool = pool.clone();
                        let parent_folder = page_name.clone();
                        pool.clone().execute(move || {
                            crawl(&client, &next_url_str, &output_dir, depth - 1, &visited, &pool, Some(parent_folder));
                        });
                    }
                }
            }
        }
    }
}
use url::Url;

pub fn url_to_filename(url: &str) -> String {
    let parsed = Url::parse(url).unwrap();
    let mut name = String::new();
    if let Some(host) = parsed.host_str() {
        name.push_str(host);
    }
    for seg in parsed.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap_or_default() {
        if !seg.is_empty() {
            name.push('_');
            name.push_str(seg);
        }
    }
    if name.is_empty() {
        name = "index".to_string();
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_url_to_filename_basic() {
        assert_eq!(url_to_filename("https://example.com"), "example.com");
        assert_eq!(url_to_filename("https://example.com/about"), "example.com_about");
        assert_eq!(url_to_filename("https://example.com/about/team"), "example.com_about_team");
        assert_eq!(url_to_filename("https://example.com/"), "example.com");
    }
    #[test]
    fn test_url_to_filename_empty_path() {
        assert_eq!(url_to_filename("https://example.com"), "example.com");
    }
    #[test]
    fn test_url_to_filename_complex() {
        assert_eq!(url_to_filename("https://sub.example.com/a/b"), "sub.example.com_a_b");
    }
}
