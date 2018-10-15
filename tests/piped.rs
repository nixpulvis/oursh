mod common;

#[test]
fn hello_world() {
    assert_piped_oursh!("echo hello world", "hello world\n");
}

#[test]
#[ignore]
fn hello_world_quoted() {
    assert_piped_oursh!("echo \"hello world\"", "hello world\n");
}

#[test]
fn simple_command() {
    assert_piped_oursh!("head README.md -n 1", "# oursh\n")
}

#[test]
#[ignore]  // TODO #10
fn compound_command() {
    assert_piped_oursh!("{ echo pi; }", "pi\n")
}

#[test]
#[ignore]
fn not_command() {
    assert_piped_oursh!(! "not true");
}

#[test]
#[ignore]
fn and_command() {
    assert_piped_oursh!("true && echo 1", "1\n");
    assert_piped_oursh!("false && echo 1", "");
}

#[test]
#[ignore]
fn or_command() {
    assert_piped_oursh!("true || echo 1", "");
    assert_piped_oursh!("false || echo 1", "1\n");
}

#[test]
fn subshell_command() {
    assert_piped_oursh!("( true )");
    assert_piped_oursh!("(echo 1)", "1\n");
}

#[test]
fn pipeline_command() {
    assert_piped_oursh!("echo pi | wc -c", "3\n");
}

#[test]
#[ignore]
fn background_command() {
    assert_piped_oursh!("sleep 1 & echo 1", "1\n");
    // TODO: I'm thinking the Job status should go to STDERR.
}

#[test]
#[cfg(feature = "bridge")]
fn bridged_sh_command() {
    assert_piped_oursh!("{#!/bin/sh; echo '1'}", "1\n");
    assert_piped_oursh!(r#"
{#!/bin/sh;
    for i in 1 2 3 4 5
    do
        echo -n $i
    done
}"#, "12345");
}

#[test]
#[cfg(feature = "bridge")]
fn bridged_ruby_command() {
    assert_piped_oursh!("{#!/usr/bin/env ruby; puts 1}", "1\n");
}

#[test]
#[cfg(feature = "bridge")]
fn bridged_python_command() {
    assert_piped_oursh!("{#!/usr/bin/env python; print(1)}", "1\n");
    assert_piped_oursh!("{#!/usr/bin/env python  ;    print(1)}", "1\n");
    assert_piped_oursh!(r#"
{#!/usr/bin/env python;
print("hello world")
}"#, "hello world\n");
}

#[test]
#[cfg(feature = "bridge")]
fn bridged_racket_command() {
    assert_piped_oursh!(r#"
{#!/usr/bin/env racket;
    #lang racket/base
    (print "hello world!")
}"#, "\"hello world!\"");
}

#[test]
#[ignore]
#[cfg(feature = "bridge")]
fn bridged_rust_command() {
    assert_piped_oursh!(r#"
{#!/usr/bin/env cargo-script-run;
    fn main() {
        println!("hello world!");
    }
}"#, "hello world!\n");
}
