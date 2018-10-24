#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path="../tests/common/mod.rs"]
mod common;

fn piped_benchmark(c: &mut Criterion) {
    c.bench_function("empty", |b| {
        b.iter(|| {
            oursh_release!("");
        })
    });
    c.bench_function("parse_error", |b| {
        b.iter(|| {
            oursh_release!("%!");
        })
    });

    c.bench_function("simple", |b| {
        b.iter(|| {
            oursh_release!("echo 1");
        })
    });

    c.bench_function("builtin", |b| {
        b.iter(|| {
            oursh_release!(":");
        })
    });

    c.bench_function("chained", |b| {
        b.iter(|| {
            oursh_release!("false; true; echo 1");
        })
    });

    c.bench_function("compound", |b| {
        b.iter(|| {
            oursh_release!("{echo 1; echo 2;}");
        })
    });

    c.bench_function("boolean", |b| {
        b.iter(|| {
            oursh_release!("! true || true && false");
        })
    });

    c.bench_function("conditional", |b| {
        b.iter(|| {
            oursh_release!("if true; then echo 1; else echo 2; fi");
        })
    });

    c.bench_function("subshell", |b| {
        b.iter(|| {
            oursh_release!("(true)");
        })
    });

    c.bench_function("pipeline", |b| {
        b.iter(|| {
            oursh_release!("echo 12345 | wc -c");
        })
    });

    c.bench_function("background", |b| {
        b.iter(|| {
            oursh_release!("echo 1 &");
        })
    });
}

criterion_group!(benches, piped_benchmark);
criterion_main!(benches);
