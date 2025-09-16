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
