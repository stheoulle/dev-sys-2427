
use clap::Parser;
use std::path::PathBuf;
use dev_sys_2427::{ crawl};

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
use std::collections::{HashSet};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;


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
