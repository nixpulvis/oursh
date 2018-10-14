use std::io::Write;
use std::process::{Command, Output, Stdio};

// Run `oursh` and collect it's output for testing... Mmmmm testing.
macro_rules! oursh {
    ($e:expr) => {{
        let mut child = Command::new("target/debug/oursh")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("error swawning oursh process");

        {
            let stdin = child.stdin.as_mut()
                .expect("error opening stdin");
            stdin.write_all($e.as_bytes())
                .expect("error writing to stdin");
        }

        let output = child.wait_with_output()
            .expect("error reading stdout");

        output
    }}
}

#[test]
fn hello_world() {
    let Output { status, stdout, stderr } = oursh!("echo hello world");
    assert!(status.success());
    assert_eq!("hello world\n", String::from_utf8_lossy(&stdout));
    assert!(stderr.is_empty());
}

#[test]
#[should_panic]
fn hello_world_quoted() {
    let Output { status, stdout, stderr } = oursh!("echo \"hello world\"");
    assert!(!status.success());
    assert_eq!("hello world\n", String::from_utf8_lossy(&stdout));
    assert!(stderr.is_empty());
}
