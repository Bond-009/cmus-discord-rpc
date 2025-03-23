use std::process::Command;

#[test]
fn test_formatting() {
    let exit_status = Command::new("cargo")
        .args(["fmt", "--all", "--check"])
        .status()
        .expect("failed to run cargo fmt");

    assert!(exit_status.success())
}
