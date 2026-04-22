use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("tdt").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn sync_dry_run_does_not_require_notion_configuration() {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = Command::cargo_bin("tdt").unwrap();
    cmd.args([
        "--data-dir",
        dir.path().to_str().unwrap(),
        "sync",
        "--dry-run",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Sync dry-run"))
    .stdout(predicate::str::contains("modo push"));
}

#[test]
fn sync_pull_dry_run_reports_pull_mode() {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = Command::cargo_bin("tdt").unwrap();
    cmd.args([
        "--data-dir",
        dir.path().to_str().unwrap(),
        "sync",
        "--dry-run",
        "--pull",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("modo pull"));
}

#[test]
fn sync_dry_run_reports_queue_counters() {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = Command::cargo_bin("tdt").unwrap();
    cmd.args([
        "--data-dir",
        dir.path().to_str().unwrap(),
        "sync",
        "--dry-run",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("fila"))
    .stdout(predicate::str::contains("pend."));
}

#[test]
fn uninstall_requires_yes() {
    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("tasks.json");
    std::fs::write(&marker, "[]").unwrap();

    let mut cmd = Command::cargo_bin("tdt").unwrap();
    cmd.args(["--data-dir", dir.path().to_str().unwrap(), "uninstall"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--yes"));

    assert!(marker.exists());
}
