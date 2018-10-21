#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

#[path="../tests/common/mod.rs"]
mod common;

#[bench]
fn oursh_script(b: &mut Bencher) {
    b.iter(|| {
        oursh_release!(> "scripts/hello_world.sh");
    })
}

#[bench]
fn sh_script(b: &mut Bencher) {
    b.iter(|| {
        shell!(> "/bin/sh", "scripts/hello_world.sh")
    })
}

#[bench]
fn zsh_script(b: &mut Bencher) {
    b.iter(|| {
        shell!(> "/usr/bin/zsh", "scripts/hello_world.sh")
    })
}

#[bench]
fn fish_script(b: &mut Bencher) {
    b.iter(|| {
        shell!(> "/usr/bin/fish", "scripts/hello_world.sh")
    })
}
