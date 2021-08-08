extern crate chrono;

#[cfg(feature = "shebang-block")]
use chrono::Local;

mod common;

#[test]
fn hello_world() {
    assert_oursh!(> "./scripts/hello_world.sh",
                    "hello world\n");
}

#[test]
fn multiline() {
    assert_oursh!(> "./scripts/multiline.sh", "12\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn date() {
    let date = Local::now().format("%Y-%m-%d").to_string();
    assert_oursh!(> "./scripts/date.oursh",
                    format!("{}\n", date));
}

#[test]
#[cfg(feature = "shebang-block")]
fn sh() {
    assert_oursh!(> "./scripts/sh.oursh", "hello world 3.14156\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn ruby() {
    assert_oursh!(> "./scripts/ruby.oursh", "hello world 3.141592653589793\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn node() {
    assert_oursh!(> "./scripts/node.oursh", "hello world 3.141592653589793\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn python() {
    assert_oursh!(> "./scripts/python.oursh", "hello world 3.141592653589793\n");
}

#[test]
#[cfg(feature = "shebang-block")]
// XXX: https://github.com/nixpulvis/oursh/issues/43
#[ignore]
fn fib() {
    assert_oursh!(> "./scripts/fib.oursh", "21\n");
}
