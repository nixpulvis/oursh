mod common;

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
