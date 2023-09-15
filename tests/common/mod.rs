#[macro_export]
macro_rules! shell {
    ($executable:expr, $args:expr, $text:expr) => {{
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new($executable)
            .args($args)
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
    }};
    (> $executable:expr, $args:expr, $filename:expr) => {{
        use std::process::{Command, Stdio};

        let child = Command::new($executable)
            .arg($filename)
            .args($args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("error swawning oursh process");

        let output = child.wait_with_output()
            .expect("error reading stdout");

        output
    }};
}

#[macro_export]
macro_rules! oursh_release {
    ($text:expr) => {{
        shell!("target/release/oursh", &["--noprofile"], $text)
    }};
    // Run `oursh` on a script file argument and collect it's output.
    (> $filename:expr) => {{
        shell!(> "target/release/oursh", &["--noprofile"], $filename)
    }};
}

/// Run `oursh` and collect it's output for testing... Mmmmm testing.
#[macro_export]
macro_rules! oursh {
    ($text:expr) => {{
        shell!("target/debug/oursh", &["--noprofile"], $text)
    }};
    // Run `oursh` on a script file argument and collect it's output.
    (> $filename:expr) => {{
        shell!(> "target/debug/oursh", &["--noprofile"], $filename)
    }};
}

#[macro_export]
macro_rules! sh {
    ($text:expr) => {{
        shell!("sh", &["--noprofile"], $text)
    }};
}

#[macro_export]
macro_rules! assert_oursh {
    (! $text:expr) => {{
        use std::process::Output;

        let Output { status, .. } = oursh!($text);
        assert!(!status.success());
    }};
    ($text:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
    }};
    ($text:expr, $stdout:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
    }};
    ($text:expr, $stdout:expr, $stderr:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!($text);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
        assert_eq!($stderr, stderr);
    }};
    (> ! $filename:expr) => {{
        use std::process::Output;

        let Output { status, .. } = oursh!(> $filename);
        assert!(!status.success());
    }};
    (> $filename:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!($filename);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert!(stderr.is_empty());
    }};
    (> $filename:expr, $stdout:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!(> $filename);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
        assert!(stderr.is_empty());
    }};
    (> $filename:expr, $stdout:expr, $stderr:expr) => {{
        use std::process::Output;

        let Output { status, stdout, stderr } = oursh!($filename);
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        println!("stdout: {}\nstderr: {}", stdout, stderr);
        assert!(status.success());
        assert_eq!($stdout, stdout);
        assert_eq!($stderr, stderr);
    }};
}

#[macro_export]
macro_rules! assert_posix {
    (! $text:expr) => {{
        use std::process::Output;

        let Output { status: oursh_status, .. } = oursh!($text);
        let Output { status: sh_status, .. } = sh!($text);
        assert_eq!(sh_status, oursh_status);
    }};
    ($text:expr) => {{
        use std::process::Output;

        let Output { status: oursh_status, stdout: oursh_stdout, stderr: oursh_stderr } = oursh!($text);
        let oursh_stdout = String::from_utf8_lossy(&oursh_stdout);
        let oursh_stderr = String::from_utf8_lossy(&oursh_stderr);
        let Output { status: sh_status, stdout: sh_stdout, stderr: sh_stderr } = sh!($text);
        let sh_stdout = String::from_utf8_lossy(&sh_stdout);
        let sh_stderr = String::from_utf8_lossy(&sh_stderr);
        assert!(oursh_status.success());
        assert_eq!(sh_status, oursh_status);
        assert_eq!(sh_stdout, oursh_stdout);
        assert_eq!(sh_stderr, oursh_stderr);
    }};
    ($text:expr, $stdout:expr) => {{
        use std::process::Output;

        let Output { status: oursh_status, stdout: oursh_stdout, stderr: oursh_stderr } = oursh!($text);
        let oursh_stdout = String::from_utf8_lossy(&oursh_stdout);
        let oursh_stderr = String::from_utf8_lossy(&oursh_stderr);
        let Output { status: sh_status, stdout: sh_stdout, stderr: sh_stderr } = sh!($text);
        let sh_stdout = String::from_utf8_lossy(&sh_stdout);
        let sh_stderr = String::from_utf8_lossy(&sh_stderr);
        println!("oursh_stdout: {}\noursh_stderr: {}", oursh_stdout, oursh_stderr);
        println!("sh_stdout: {}\nsh_stderr: {}", sh_stdout, sh_stderr);
        assert_eq!(sh_status, oursh_status);
        assert_eq!(sh_stdout, oursh_stdout);
        assert_eq!(sh_stderr, oursh_stderr);
        assert_eq!($stdout, oursh_stdout);
    }};
}
