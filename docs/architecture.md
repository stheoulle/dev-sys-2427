# Project Architecture: Webcrawl

## Project Definition

**Webcrawl** is a multithreaded command-line web crawler written in Rust. Its goal is to download and process web pages from a given list of starting URLs, following links up to a specified depth, and storing all scraped pages locally in a hierarchical folder structure that reflects the crawl order. This tool is designed for fast, concurrent scraping and easy offline analysis of website structures.

## Components and Modules

- **Argument Parser (`clap`)**: Handles command-line arguments for output directory, crawl depth, and starting URLs.
- **Crawler Core (`crawl` function)**: Manages the recursive crawling logic, including downloading pages, parsing links, and controlling crawl depth.
- **Thread Pool (`threadpool`)**: Enables concurrent crawling of multiple pages for performance and efficiency.
- **HTTP Client (`reqwest`)**: Downloads HTML content from web pages.
- **HTML Parser (`scraper`)**: Extracts links from downloaded HTML to discover new pages to crawl.
- **Hierarchical Saver**: Ensures each page is saved in a folder structure that mirrors the crawl hierarchy (parent-child relationship).
- **Visited Set**: Tracks URLs already crawled to avoid duplicates and infinite loops.

This architecture separates concerns for clarity and maintainability. Using a thread pool allows efficient resource usage, while modular functions make the codebase easy to extend and debug.

## Usage

Build the project with Cargo:

```bash
cargo build --release
```

Run the crawler:

```bash
cargo run webcrawl --output ./crawled_url --depth 3 https://example.com
```

- `--output`: Directory where crawled pages will be saved.
- `--depth`: Maximum link-following depth.
- `<URL>`: One or more starting URLs.

Example output structure:

```bash
crawled_url/
  |- example_com.html
  |- example_com/
      |- about.html
      |- about/
          |- team.html
```
