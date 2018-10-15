#[macro_export]
// Run `oursh` and collect it's output for testing... Mmmmm testing.
macro_rules! piped_oursh {
    ($text:expr) => {{
        use std::io::Write;
        use std::process::{Command, Stdio};

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

#[macro_export]
macro_rules! assert_piped_oursh {
    (! $text:expr) => {{
        use std::process::Output;

        let Output { status, .. } = piped_oursh!($text);
        assert!(!status.success());
    }};
    ($text:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = piped_oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = piped_oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr, $stderr:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = piped_oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
        assert_eq!($stderr, stderr);
    }};
}

#[macro_export]
// Run `oursh` on a script file argument and collect it's output.
macro_rules! script_oursh {
    ($filename:expr) => {{
        use std::process::{Command, Stdio};

        let mut child = Command::new("target/debug/oursh")
            .arg($filename)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("error swawning oursh process");

        let output = child.wait_with_output()
            .expect("error reading stdout");

        output
    }}
}

#[macro_export]
macro_rules! assert_script_oursh {
    (! $text:expr) => {{
        use std::process::Output;

        let Output { status, .. } = script_oursh!($text);
        assert!(!status.success());
    }};
    ($text:expr) => {{
        use std::process::Output;

        let Output { status, stderr, .. } = script_oursh!($text);
        assert!(status.success());
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = script_oursh!($text);
        assert!(status.success());
        assert_eq!($stdout, String::from_utf8_lossy(&stdout));
        assert!(stderr.is_empty());
    }};
    ($text:expr, $stdout:expr, $stderr:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = script_oursh!($text);
        assert!(status.success());
        assert_eq!($stdout, String::from_utf8_lossy(&stdout));
        assert_eq!($stderr, String::from_utf8_lossy(&stderr));
    }};
}
