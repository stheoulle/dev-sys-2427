
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for crawled pages
    #[arg(short, long)]
    output: PathBuf,

    /// Maximum crawl depth
    #[arg(short, long, default_value_t = 2)]
    depth: usize,

    /// Starting URLs
    #[arg(required = true)]
    urls: Vec<String>,
}

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs;
use std::collections::{HashSet};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use url::Url;

fn crawl(
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
                fs::create_dir_all(&d).ok();
                d
            } else {
                output_dir.clone()
            };
            let mut save_path = save_dir.clone();
            save_path.push(format!("{}.html", page_name));
            match fs::write(&save_path, &body) {
                Ok(_) => println!("Saved: {}", save_path.display()),
                Err(e) => eprintln!("Failed to save {}: {}", save_path.display(), e),
            }

            // Parse links
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a[href]").unwrap();
            for element in document.select(&selector) {
                if let Some(link) = element.value().attr("href") {
                    if let Ok(next_url) = Url::parse(link).or_else(|_| Url::parse(&format!("{}{}", url, link))) {
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

fn url_to_filename(url: &str) -> String {
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

fn main() {
    let args = Args::parse();
    let client = Client::new();
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let pool = ThreadPool::new(8);
    for url in &args.urls {
        let url = url.clone();
        let visited = Arc::clone(&visited);
        let output_dir = args.output.clone();
        let client = client.clone();
        let pool = pool.clone();
        pool.clone().execute(move || {
            crawl(&client, &url, &output_dir, args.depth, &visited, &pool, None);
        });
    }
    pool.join();
}
