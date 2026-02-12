use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn bin_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("ftrek"))
}

#[test]
fn prints_tree_for_explicit_root() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();
    fs::create_dir(root.join("nested")).expect("create nested dir");
    fs::write(root.join("nested").join("file.txt"), "hello").expect("write file");

    let root_str = root.to_string_lossy().to_string();

    let mut cmd = bin_cmd();
    cmd.arg(&root_str);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{root_str}/")))
        .stdout(predicate::str::contains("nested/"))
        .stdout(predicate::str::contains("file.txt"));
}

#[test]
fn applies_gitignore_filtering() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();

    fs::write(root.join(".gitignore"), "ignored.txt\nignored_dir/\n").expect("write gitignore");
    fs::write(root.join("kept.txt"), "kept").expect("write kept file");
    fs::write(root.join("ignored.txt"), "ignored").expect("write ignored file");
    fs::create_dir(root.join("ignored_dir")).expect("create ignored dir");
    fs::write(root.join("ignored_dir").join("inside.txt"), "inside").expect("write nested");

    let root_str = root.to_string_lossy().to_string();

    let mut cmd = bin_cmd();
    cmd.arg("--gitignore").arg(&root_str);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("kept.txt"))
        .stdout(predicate::str::contains("ignored.txt").not())
        .stdout(predicate::str::contains("ignored_dir").not())
        .stdout(predicate::str::contains("inside.txt").not());
}

#[test]
fn defaults_to_current_directory_when_no_root_is_passed() {
    let temp = tempdir().expect("create tempdir");
    fs::write(temp.path().join("local.txt"), "local").expect("write file");

    let mut cmd = bin_cmd();
    cmd.current_dir(temp.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("./"))
        .stdout(predicate::str::contains("local.txt"));
}

#[test]
fn prints_help_documentation() {
    let mut cmd = bin_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("[DIRECTORY]"))
        .stdout(predicate::str::contains("--gitignore"));
}
