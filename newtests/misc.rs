use regex::Regex;

#[test]
fn unclosed_group_error() {
    let err = Regex::new(r"(").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("unclosed group"), "error message: {:?}", msg);
}
