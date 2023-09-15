mod common;

#[test]
fn hello_world() {
    assert_posix!("echo hello world", "hello world\n");
}

#[test]
fn builtin_cd() {
    assert_posix!("cd /; pwd", "/\n");
    // assert_posix!("cd; pwd", "$HOME\n");
    // assert_posix!("cd ~; pwd", "$HOME\n");
    // assert_posix!("cd /; cd /home; cd -", "/\n");
}

#[test]
fn builtin_exit() {
    assert_posix!("exit");
    assert_posix!(! "exit 1");
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
    assert_posix!("echo 'hello world'", "hello world\n");
    assert_posix!("echo \"hello world\"", "hello world\n");
}

#[test]
fn simple_command() {
    assert_posix!("head README.md -n 1", "# oursh\n");
}

#[test]
fn chained_command() {
    assert_posix!("false; true; echo 1", "1\n");
    assert_posix!("true; false; echo 2;", "2\n");
}

#[test]
fn single_compound_command() {
    assert_posix!("{ echo pi; }", "pi\n");
    assert_posix!("{echo pi; }");  // NOTE: Fails in sh
    // assert_posix!("{echo pi}");    // NOTE: Allowed in zsh
}

#[test]
fn multiple_compound_command() {
    assert_posix!("{ echo pi; echo e; }", "pi\ne\n");
    assert_posix!("{ FOO=1; }; echo $FOO", "1\n");
}

#[test]
#[ignore]
fn multiple_tee_command() {
    // TODO: Might need a way to test order independent output.
    assert_posix!("echo foo | tee >(wc -c) | base64", "4\nZm9vCg==")
}

#[test]
fn not_command() {
    assert_posix!(! "! true");
    assert_posix!(! "! true && echo 1");
}

#[test]
fn and_command() {
    assert_posix!("true && echo 1", "1\n");
    assert_posix!( !"false && echo 1");
}

#[test]
fn or_command() {
    assert_posix!("true || echo 1", "");
    assert_posix!("false || echo 1", "1\n");
}

#[test]
fn cond_command() {
    assert_posix!("if true; then echo 1; else echo 2; fi", "1\n");
    assert_posix!("if false; then echo 1; else echo 2; fi", "2\n");
    assert_posix!("if false; then echo 1; elif false; then echo 2; else echo 3; fi", "3\n");
    assert_posix!("if false; then echo 1; elif true; then echo 2; else echo 3; fi", "2\n");
}

#[test]
fn subshell_command() {
    assert_posix!("$( true )");
    assert_posix!("$(echo 1)", "1\n");
    assert_posix!("$(false; echo 1)", "1\n");
    // TODO: Test some actual subshell usage.
}

#[test]
fn single_pipeline_command() {
    assert_posix!("echo pi | wc -c", "3\n");
}

#[test]
#[ignore]
fn chained_pipeline_command() {
    assert_posix!("cat README.md | head | wc -l", "10\n");
}

#[test]
#[ignore]
fn assignment_command() {
    assert_posix!("PI=3.1415 printenv PI", "3.1415\n");
    assert_posix!("X=1 Y=2 printenv X Y", "1\n2\n");
    assert_posix!("X=1; printenv X", "\n");
}

#[test]
fn variable_command() {
    assert_posix!("X=1; echo $X", "1\n");
    assert_posix!("X=1 echo $X", "\n");
    assert_posix!("X=1; printenv X", "\n");
    assert_posix!("X=1 printenv X", "1\n");

    assert_posix!("export FOO=1 BAR=$FOO; echo $BAR", "1\n");

    assert_posix!("echo $BAR", "\n");
    assert_posix!("echo $", "$\n");
    assert_posix!("echo ' $ '", " $ \n");
    assert_posix!("echo \" $$ $ \"", " $$ $ \n");
}

#[test]
fn background_command() {
    assert_posix!("sleep 1 & echo 1", "1\n");
    // TODO: How to test the output with a PID in it?
    // assert_posix!("sleep 1 & echo 1", "1\n", "[1]\t(\d*)\n");
}
