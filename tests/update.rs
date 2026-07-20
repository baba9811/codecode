use practicode::update::{is_newer, update_is_eligible};

#[test]
fn version_compare_detects_patch_updates() {
    assert!(is_newer("0.1.2", "0.1.1"));
    assert!(is_newer("0.2.0", "0.1.9"));
    assert!(!is_newer("0.1.1", "0.1.1"));
    assert!(!is_newer("0.1.0", "0.1.1"));
}

#[test]
fn malformed_or_prerelease_versions_do_not_trigger_an_update() {
    assert!(!is_newer("999.invalid", "0.2.0"));
    assert!(!is_newer("1.0", "0.2.0"));
    assert!(!is_newer("1.0.0-beta.1", "0.2.0"));
}

#[test]
fn skipped_version_waits_for_a_newer_release() {
    assert!(!update_is_eligible("0.2.4", "0.2.3", "0.2.4"));
    assert!(update_is_eligible("0.2.5", "0.2.3", "0.2.4"));
    assert!(update_is_eligible("0.2.4", "0.2.3", ""));
}
