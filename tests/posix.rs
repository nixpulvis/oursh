use std::str;

mod common;

macro_rules! assert_posix {
    ($text:expr) => {{
        use std::process::Output;

        let sh = shell!("/usr/bin/sh", [] as [&str; 0], $text);
        let oursh = oursh!($text);
        assert_eq!(sh.status, oursh.status);
        assert_eq!(str::from_utf8(&sh.stdout), str::from_utf8(&oursh.stdout));
        assert_eq!(str::from_utf8(&sh.stderr), str::from_utf8(&oursh.stderr));
    }};
    (< $filename:expr) => {{
        let sh = shell!(< "/usr/bin/sh", [] as [&str; 0], $filename);
        let oursh = oursh!(< $filename);
        assert_eq!(sh.status, oursh.status);
        assert_eq!(sh.stdout.to_string_lossy(), oursh.stdout.to_string_lossy());
        assert_eq!(sh.stderr.to_string_lossy(), oursh.stderr.to_string_lossy());
    }};
}

#[test]
fn hello_world() {
    assert_posix!("true");
    assert_posix!("echo hello world");
}

#[test]
fn builtin_cd() {
    assert_posix!("cd /; pwd");
    assert_posix!("cd; pwd");
    assert_posix!("cd ~; pwd");
    // TODO: cd -
    // assert_posix!("cd /; cd /home; cd -");
}

#[test]
fn builtin_exit() {
    assert_posix!("exit");
}

#[test]
fn builtin_null() {
    assert_posix!(":");
}

#[test]
#[ignore]
fn forkbomb() {
    assert_posix!(":(){ :|: & };:");
}

#[test]
fn hello_world_quoted() {
    assert_posix!("echo 'hello world'");
    assert_posix!("echo \"hello world\"");
}

#[test]
fn simple_command() {
    assert_posix!("head README.md -n 1");
}

#[test]
fn chained_command() {
    assert_posix!("false; true; echo 1");
    assert_posix!("true; false; echo 2;");
}

#[test]
fn single_compound_command() {
    assert_posix!("{ echo pi; }");
    assert_oursh!("{echo pi; }");  // NOTE: Fails in sh
    // assert_oursh!("{echo pi}");    // NOTE: Allowed in zsh
}

#[test]
fn multiple_compound_command() {
    assert_posix!("{ echo pi; echo e; }");
    assert_posix!("{ FOO=1; }; echo $FOO");
}

#[test]
// #[ignore]
fn multiple_tee_command() {
    // TODO: Might need a way to test order independent output.
    // assert_oursh!("echo foo | tee >(wc -c) | base64", "4\nZm9vCg==");
    assert_posix!("echo foo | tee >(wc -c) | base64");
}

#[test]
fn not_command() {
    assert_posix!("! true");
    assert_posix!("! true && echo 1");
}

#[test]
fn and_command() {
    assert_posix!("true && echo 1");
    assert_posix!("false && echo 1");
}

#[test]
fn or_command() {
    assert_posix!("true || echo 1");
    assert_posix!("false || echo 1");
}

#[test]
fn cond_command() {
    assert_posix!("if true; then echo 1; else echo 2; fi");
    assert_posix!("if false; then echo 1; else echo 2; fi");
    assert_posix!("if false; then echo 1; elif false; then echo 2; else echo 3; fi");
    assert_posix!("if false; then echo 1; elif true; then echo 2; else echo 3; fi");
}

#[test]
fn subshell_command() {
    assert_posix!("$( true )");
    assert_posix!("$(echo 1)");
    assert_posix!("$(false; echo 1)");
    // TODO: Test some actual subshell usage.
}

#[test]
fn single_pipeline_command() {
    assert_posix!("echo pi | wc -c");
}

#[test]
// #[ignore]
fn chained_pipeline_command() {
    assert_posix!("cat README.md | head | wc -l");
}

#[test]
// #[ignore]
fn assignment_command() {
    assert_posix!("PI=3.1415 printenv PI");
    assert_posix!("X=1 Y=2 printenv X Y");
    assert_posix!("X=1; printenv X");
}

#[test]
fn variable_command() {
    assert_posix!("X=1; echo $X");
    assert_posix!("export FOO=1 BAR=$FOO; echo $BAR");
    assert_posix!("echo $BAR");
    assert_posix!("echo $");
    assert_posix!("echo ' $ '");
    assert_posix!("echo \" $$ $ \"");
}

#[test]
fn background_command() {
    assert_posix!("sleep 1 & echo 1");
    // TODO: How to test the output with a PID in it?
    // assert_oursh!("sleep 1 & echo 1", "1\n", "[1]\t(\d*)\n");
}
