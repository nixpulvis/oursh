#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path="../tests/common/mod.rs"]
mod common;

fn piped_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("oursh");

    group.bench_function("empty", |b| {
        b.iter(|| {
            oursh_release!("")
        })
    });

    group.bench_function("parse_error", |b| {
        b.iter(|| {
            oursh_release!("%!");
        })
    });

    group.bench_function("simple", |b| {
        b.iter(|| {
            oursh_release!("echo 1");
        })
    });

    group.bench_function("builtin", |b| {
        b.iter(|| {
            oursh_release!(":");
        })
    });

    group.bench_function("chained", |b| {
        b.iter(|| {
            oursh_release!("false; true; echo 1");
        })
    });

    group.bench_function("compound", |b| {
        b.iter(|| {
            oursh_release!("{echo 1; echo 2;}");
        })
    });

    group.bench_function("boolean", |b| {
        b.iter(|| {
            oursh_release!("! true || true && false");
        })
    });

    group.bench_function("conditional", |b| {
        b.iter(|| {
            oursh_release!("if true; then echo 1; else echo 2; fi");
        })
    });

    group.bench_function("subshell", |b| {
        b.iter(|| {
            oursh_release!("(true)");
        })
    });

    group.bench_function("pipeline", |b| {
        b.iter(|| {
            oursh_release!("echo 12345 | wc -c");
        })
    });

    group.bench_function("background", |b| {
        b.iter(|| {
            oursh_release!("echo 1 &");
        })
    });

    group.finish();

    #[cfg(feature = "shebang-block")]
    {
        let mut group = c.benchmark_group("oursh shebang-block");

        group.bench_function("ruby", |b| {
            b.iter(|| {
                oursh_release!("{#!ruby; puts 1}");
            })
        });

        group.bench_function("node", |b| {
            b.iter(|| {
                oursh_release!("{#!node; console.log(1)}");
            })
        });

        group.bench_function("python", |b| {
            b.iter(|| {
                oursh_release!("{#!python; print(1)}");
            })
        });

        group.bench_function("sh", |b| {
            b.iter(|| {
                oursh_release!("{#!/bin/sh; echo 1}");
            })
        });

        group.finish();
    }
}

criterion_group!(benches, piped_benchmark);
criterion_main!(benches);
