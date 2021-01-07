use assert_cmd::Command;

#[test]
fn test_default_command() {
    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    cmd.arg("./fixtures/different_versions").assert().success();
}

#[test]
fn test_default_command_without_mismatches() {
    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    cmd.arg("./fixtures/same_versions").assert().success();
}

#[test]
fn test_check_command() {
    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    let assert = cmd
        .arg("./fixtures/different_versions")
        .arg("check")
        .assert();
    assert.code(1);
}

#[test]
fn test_print_command() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    let assert = cmd
        .arg("./fixtures/different_versions")
        .arg("print")
        .assert();
    assert.success();
}
