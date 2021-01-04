use assert_cmd::Command;

#[test]
fn test_default_command() {
    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    cmd.assert().success();
}

#[test]
fn test_check_command() {
    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    let assert = cmd.arg("check").assert();
    assert.code(1);
}

#[test]
fn test_print_command() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("rubsty").unwrap();
    let assert = cmd.arg("print").assert();
    assert.success();
}
