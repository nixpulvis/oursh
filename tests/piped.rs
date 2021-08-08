mod common;

#[test]
fn hello_world() {
    assert_oursh!("echo hello world", "hello world\n");
}

#[test]
fn builtin_cd() {
    assert_oursh!("cd /; pwd", "/\n");
    // assert_oursh!("cd; pwd", "$HOME\n");
    // assert_oursh!("cd ~; pwd", "$HOME\n");
    // assert_oursh!("cd /; cd /home; cd -", "/\n");
}

#[test]
fn builtin_exit() {
    assert_oursh!("exit");
    assert_oursh!(! "exit 1");
}

#[test]
fn builtin_null() {
    assert_oursh!(":");
}

#[test]
#[ignore]
fn forkbomb() {
    assert_oursh!(":(){ :|: & };:");
}

#[test]
fn hello_world_quoted() {
    assert_oursh!("echo 'hello world'", "hello world\n");
    assert_oursh!("echo \"hello world\"", "hello world\n");
}

#[test]
fn simple_command() {
    assert_oursh!("head README.md -n 1", "# oursh\n");
}

#[test]
fn chained_command() {
    assert_oursh!("false; true; echo 1", "1\n");
    assert_oursh!("true; false; echo 2;", "2\n");
}

#[test]
fn single_compound_command() {
    assert_oursh!("{ echo pi; }", "pi\n");
    // assert_oursh!("{ echo pi }");  // TODO: write test for fail to parse
}

#[test]
fn multiple_compound_command() {
    assert_oursh!("{ echo pi; echo e; }", "pi\ne\n");
}

#[test]
#[ignore]
fn multiple_tee_command() {
    // TODO: Might need a way to test order independent output.
    assert_oursh!("echo foo | tee >(wc -c) | base64", "4\nZm9vCg==")
}

#[test]
fn not_command() {
    assert_oursh!(! "! true");
    assert_oursh!(! "! true && echo 1");
}

#[test]
fn and_command() {
    assert_oursh!("true && echo 1", "1\n");
    assert_oursh!( !"false && echo 1");
}

#[test]
fn or_command() {
    assert_oursh!("true || echo 1", "");
    assert_oursh!("false || echo 1", "1\n");
}

#[test]
fn cond_command() {
    assert_oursh!("if true; then echo 1; else echo 2; fi", "1\n");
    assert_oursh!("if false; then echo 1; else echo 2; fi", "2\n");
    assert_oursh!("if false; then echo 1; elif false; then echo 2; else echo 3; fi", "3\n");
    assert_oursh!("if false; then echo 1; elif true; then echo 2; else echo 3; fi", "2\n");
}

#[test]
fn subshell_command() {
    assert_oursh!("( true )");
    assert_oursh!("(echo 1)", "1\n");
    assert_oursh!("(false; echo 1)", "1\n");
    // TODO: Test some actual subshell usage.
}

#[test]
fn single_pipeline_command() {
    assert_oursh!("echo pi | wc -c", "3\n");
}

#[test]
#[ignore]
fn chained_pipeline_command() {
    assert_oursh!("cat README.md | head | wc -l", "10\n");
}

#[test]
#[ignore]
fn assignment_command() {
    assert_oursh!("PI=3.1415 printenv PI", "3.1415\n");
    assert_oursh!("X=1 Y=2 printenv X Y", "1\n2\n");
    assert_oursh!("X=1; printenv X", "\n");
}

#[test]
#[ignore]
fn variable_command() {
    assert_oursh!("X=1; echo $X", "1\n");
}

#[test]
fn background_command() {
    assert_oursh!("sleep 1 & echo 1", "1\n");
    // TODO: How to test the output with a PID in it?
    // assert_oursh!("sleep 1 & echo 1", "1\n", "[1]\t(\d*)\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn shebang_block_sh_command() {
    assert_oursh!("{#!/bin/sh; echo '1'}", "1\n");
    assert_oursh!(r#"{#!/bin/sh;
    for i in 1 2 3 4 5
    do
        echo -n $i
    done
}"#, "12345");
}

#[test]
#[cfg(feature = "shebang-block")]
fn shebang_block_ruby_command() {
    assert_oursh!("{#!/usr/bin/env ruby; puts 1}", "1\n");
}

#[test]
#[cfg(feature = "shebang-block")]
fn shebang_block_python_command() {
    assert_oursh!("{#!/usr/bin/env python; print(1)}", "1\n");
    assert_oursh!("{#!/usr/bin/env python  ;    print(1)}", "1\n");
    assert_oursh!(r#"{#!/usr/bin/env python;
print("hello world")
}"#, "hello world\n");
}

#[test]
#[ignore]
#[cfg(feature = "shebang-block")]
fn shebang_block_racket_command() {
    assert_oursh!(r#"{#!/usr/bin/env racket;
    #lang racket/base
    (print "hello world!")
}"#, "\"hello world!\"");
}

#[test]
#[ignore]
#[cfg(feature = "shebang-block")]
fn shebang_block_rust_command() {
    assert_oursh!(r#"{#!/usr/bin/env cargo-script-run;
    fn main() {
        println!("hello world!");
    }
}"#, "hello world!\n");
}
