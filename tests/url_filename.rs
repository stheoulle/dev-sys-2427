use dev_sys_2427::url_to_filename;

#[test]
fn test_url_to_filename_integration() {
    assert_eq!(url_to_filename("https://example.com"), "example.com");
    assert_eq!(url_to_filename("https://example.com/about"), "example.com_about");
    assert_eq!(url_to_filename("https://sub.example.com/a/b"), "sub.example.com_a_b");
}
