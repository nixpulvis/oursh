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
            shell!(> "/bin/sh", s)
        })
    });

    let zsh = Fun::new("zsh", |b, s| {
        b.iter(|| {
            shell!(> "/usr/bin/zsh", s)
        })
    });

    let fish = Fun::new("fish", |b, s| {
        b.iter(|| {
            shell!(> "/usr/bin/fish", s)
        });
    });

    let benches = vec![oursh, sh, zsh, fish];

    let home = env::var("HOME").expect("HOME not set");
    let config = format!("{}/.config/fish/config.fish", home);
    let backup = format!("{}.old", &config);
    rename(&config, &backup).expect("save fish config");
    println!("moved {}", &config);
    c.bench_functions("hello world", benches, "scripts/hello_world.sh");
    rename(&backup, &config).expect("restore fish config");
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
