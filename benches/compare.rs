#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

#[bench]
fn oursh_script(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("target/release/oursh")
            .arg("scripts/hello_world.sh")
            .output()
            .expect("failed to run sh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn sh_script(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("/bin/sh")
            .arg("scripts/hello_world.sh")
            .output()
            .expect("failed to run sh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn zsh_script(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("/usr/bin/zsh")
            .arg("scripts/hello_world.sh")
            .output()
            .expect("failed to run sh")
            .status;
        assert!(status.success());
    })
}

#[bench]
fn fish_script(b: &mut Bencher) {
    b.iter(|| {
        let status = Command::new("/bin/fish")
            .arg("scripts/hello_world.sh")
            .output()
            .expect("failed to run fish")
            .status;
        assert!(status.success());
    })
}
