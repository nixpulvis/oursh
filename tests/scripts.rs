use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn hello_world() {
    let mut child = Command::new("target/debug/oursh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("error swawning oursh process");

    {
        let stdin = child.stdin.as_mut()
            .expect("error opening stdin");
        stdin.write_all("echo hello world".as_bytes())
            .expect("error writing to stdin");
    }

    let output = child.wait_with_output()
        .expect("error reading stdout");

    assert_eq!("hello world\n", String::from_utf8_lossy(&output.stdout));
}
