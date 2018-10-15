extern crate chrono;

#[cfg(feature = "bridge")]
use chrono::Local;

mod common;

#[test]
#[cfg(feature = "bridge")]
fn date() {
    let date = Local::now().format("%Y-%m-%d").to_string();
    assert_script_oursh!("./scripts/date.oursh",
                         format!("{}\n", date));
}

#[test]
fn hello_world() {
    assert_script_oursh!("./scripts/hello_world.sh",
                         "hello world\n");
}

#[test]
#[cfg(feature = "bridge")]
#[ignore]
fn fib() {
    assert_script_oursh!("./scripts/fib.oursh", "21\n");
}

#[test]
#[ignore]  // Waiting on custom lexer.
fn multiline() {
    assert_script_oursh!("./scripts/multiline.sh", "12\n");
}
