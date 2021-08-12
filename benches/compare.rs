#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path="../tests/common/mod.rs"]
mod common;

fn compare_script_benchmark(s: &str, c: &mut Criterion) {
    let mut group = c.benchmark_group("hello world");

    group.bench_function("oursh", |b| {
        b.iter(|| {
            oursh_release!(> s)
        })
    });

    group.bench_function("sh", |b| {
        b.iter(|| {
            shell!(> "/bin/sh", [] as [&str; 0], s)
        })
    });

    group.bench_function("zsh", |b| {
        b.iter(|| {
            shell!(> "/usr/bin/zsh", [] as [&str; 0], s)
        })
    });

    group.bench_function("fish", |b| {
        b.iter(|| {
            shell!(> "/usr/bin/fish", [] as [&str; 0], s)
        });
    });

    group.finish();
}

fn compare_scripts_benchmark(c: &mut Criterion) {
    compare_script_benchmark("scripts/hello_world.sh", c);
    compare_script_benchmark("scripts/multiline.sh", c);
}

criterion_group!(benches, compare_scripts_benchmark);
criterion_main!(benches);
