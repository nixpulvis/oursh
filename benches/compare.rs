#[macro_use]
extern crate criterion;

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
        })
    });

    let benches = vec![oursh, sh, zsh, fish];

    c.bench_functions("hello world", benches, "scripts/hello_world.sh");
    c.bench_functions("multiline", benches, "scripts/multiline.sh");
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
