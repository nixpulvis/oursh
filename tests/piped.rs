use std::io::Write;
use std::process::{Command, Output, Stdio};

// Run `oursh` and collect it's output for testing... Mmmmm testing.
macro_rules! oursh {
    ($text:expr) => {{
        let mut child = Command::new("target/debug/oursh")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("error swawning oursh process");

        {
            let stdin = child.stdin.as_mut()
                .expect("error opening stdin");
            stdin.write_all($text.as_bytes())
                .expect("error writing to stdin");
        }

        let output = child.wait_with_output()
            .expect("error reading stdout");

        output
    }}
}

// Run `oursh` and assert! it's output.
macro_rules! assert_oursh {
    (! $text:expr) => {{
        let Output { status, .. } = oursh!($text);
        assert!(!status.success());
    }};
    ($text:expr) => {{
        let Output { status, stderr, .. } = oursh!($text);
        assert!(status.success());
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr) => {{
        let Output { status, stdout, stderr } = oursh!($text);
        assert!(status.success());
        assert_eq!($stdout, String::from_utf8_lossy(&stdout));
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr, $stderr:expr) => {{
        let Output { status, stdout, stderr } = oursh!($text);
        assert!(status.success());
        assert_eq!($stdout, String::from_utf8_lossy(&stdout));
        assert_eq!($stderr, String::from_utf8_lossy(&stderr));
    }};
}

#[test]
fn hello_world() {
    assert_oursh!("echo hello world", "hello world\n");
}

#[test]
#[should_panic]
fn hello_world_quoted() {
    assert_oursh!("echo \"hello world\"", "hello world\n");
}

#[test]
fn simple_command() {
    assert_oursh!("head README.md -n 1", "# oursh\n")
}

#[test]
fn compound_command() {
    assert_oursh!("{ echo pi }", "pi\n")
    // TODO: Should be this:
    // assert_oursh!("{ ls; }", "# oursh\n")
}

#[test]
#[should_panic]
fn not_command() {
    assert_oursh!(! "not true");
}

#[test]
#[should_panic]
fn and_command() {
    assert_oursh!("true && echo 1", "1\n");
    assert_oursh!("false && echo 1", "");
}

#[test]
#[should_panic]
fn or_command() {
    assert_oursh!("true || echo 1", "");
    assert_oursh!("false || echo 1", "1\n");
}

#[test]
fn subshell_command() {
    assert_oursh!("(echo 1)", "1\n");
}

#[test]
fn pipeline_command() {
    assert_oursh!("echo pi | wc -c", "3\n");
}

#[test]
#[should_panic]
fn background_command() {
    assert_oursh!("sleep 1 & echo 1", "1\n");
    // TODO: I'm thinking the Job status should go to STDERR.
}

#[test]
fn bridged_command() {
    assert_oursh!("{@#!/usr/bin/env ruby; puts 1}", "1\n");
}
