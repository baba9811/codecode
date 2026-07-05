use practicode::update::is_newer;

#[test]
fn version_compare_detects_patch_updates() {
    assert!(is_newer("0.1.2", "0.1.1"));
    assert!(is_newer("0.2.0", "0.1.9"));
    assert!(!is_newer("0.1.1", "0.1.1"));
    assert!(!is_newer("0.1.0", "0.1.1"));
}
