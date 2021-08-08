#[macro_use]
extern crate criterion;

use std::env;
use std::fs::rename;
use criterion::{Criterion, Fun};

#[path="../tests/common/mod.rs"]
mod common;

fn compare_benchmark(c: &mut Criterion) {
    let oursh = Fun::new("oursh", |b, s| {
        b.iter(|| {
            oursh_release!(> s)
        })
    });

    let sh = Fun::new("sh", |b, s| {
        b.iter(|| {
            shell!(> "/bin/sh", [] as [&str; 0], s)
        })
    });

    let zsh = Fun::new("zsh", |b, s| {
        b.iter(|| {
            shell!(> "/usr/bin/zsh", [] as [&str; 0], s)
        })
    });

    let fish = Fun::new("fish", |b, s| {
        b.iter(|| {
            shell!(> "/usr/bin/fish", [] as [&str; 0], s)
        });
    });

    let benches = vec![oursh, sh, zsh, fish];

    c.bench_functions("hello world", benches, "scripts/hello_world.sh");
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
