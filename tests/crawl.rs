use scraper::{Html, Selector};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

#[test]
fn test_visited_set_logic() {
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let url = "https://example.com";
    {
        let mut v = visited.lock().unwrap();
        assert!(!v.contains(url));
        v.insert(url.to_string());
    }
    {
        let v = visited.lock().unwrap();
        assert!(v.contains(url));
    }
}

#[test]
fn test_html_link_extraction() {
    let html = r#"
        <html>
            <body>
                <a href="https://example.com/page1">Page 1</a>
                <a href="/page2">Page 2</a>
            </body>
        </html>
    "#;
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();
    let mut links = vec![];
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(link.to_string());
        }
    }
    assert!(links.contains(&"https://example.com/page1".to_string()));
    assert!(links.contains(&"/page2".to_string()));
    assert_eq!(links.len(), 2);
}
