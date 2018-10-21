#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

#[path="../tests/common/mod.rs"]
mod common;

#[bench]
fn oursh_piped_sh(b: &mut Bencher) {
    b.iter(|| {
        assert_oursh!("{#!/bin/sh; echo 1}", "1\n");
    })
}

#[bench]
fn oursh_script_sh(b: &mut Bencher) {
    b.iter(|| {
        assert_oursh!(> "scripts/sh.oursh");
    })
}

#[bench]
fn oursh_script_ruby(b: &mut Bencher) {
    b.iter(|| {
        assert_oursh!(> "scripts/ruby.oursh");
    })
}

#[bench]
fn oursh_script_node(b: &mut Bencher) {
    b.iter(|| {
        assert_oursh!(> "scripts/node.oursh");
    })
}

#[bench]
fn oursh_script_python(b: &mut Bencher) {
    b.iter(|| {
        assert_oursh!(> "scripts/python.oursh");
    })
}
