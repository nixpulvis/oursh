#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

#[bench]
fn oursh_script_sh(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("target/release/oursh")
            .arg("scripts/sh.oursh")
            .output()
            .expect("failed to run oursh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn oursh_script_ruby(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("target/release/oursh")
            .arg("scripts/ruby.oursh")
            .output()
            .expect("failed to run oursh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn oursh_script_node(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("target/release/oursh")
            .arg("scripts/node.oursh")
            .output()
            .expect("failed to run oursh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn oursh_script_python(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("target/release/oursh")
            .arg("scripts/python.oursh")
            .output()
            .expect("failed to run oursh")
            .status;
        assert!(status.success());
    })
}
