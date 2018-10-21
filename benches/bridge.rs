#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

#[path="../tests/common/mod.rs"]
mod common;

#[bench]
fn oursh_piped_sh(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!("{#!/bin/sh; echo 1}");
    })
}

#[bench]
fn oursh_piped_ruby(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!("{#!ruby; puts 1}");
    })
}

#[bench]
fn oursh_piped_node(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!("{#!node; console.log(1)}");
    })
}

#[bench]
fn oursh_piped_python(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!("{#!python; print(1)}");
    })
}

#[bench]
fn oursh_script_sh(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!(> "scripts/sh.oursh");
    })
}

#[bench]
fn oursh_script_ruby(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!(> "scripts/ruby.oursh");
    })
}

#[bench]
fn oursh_script_node(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!(> "scripts/node.oursh");
    })
}

#[bench]
fn oursh_script_python(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!(> "scripts/python.oursh");
    })
}
