use anyhow::Result;  // Easier to use than the default Result type.
use assert_cmd::Command;
use predicates::prelude::*; 
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn dies_no_args() -> Result<()>  {
    // $ echor       ;  with no arguments, should fail
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert() 
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// Reference files are produced by mk-outs.sh, which see.

// Test runner
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    println!("expected <{}>", expected);
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    println!("got <{}>", stdout);
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["-n", "Hello there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

