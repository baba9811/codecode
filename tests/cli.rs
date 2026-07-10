use std::process::Command;

#[test]
fn version_does_not_require_a_data_home() {
    let output = Command::new(env!("CARGO_BIN_EXE_practicode"))
        .arg("--version")
        .env_remove("PRACTICODE_HOME")
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("practicode "));
}
