#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path="../tests/common/mod.rs"]
mod common;

fn shebang_block_benchmark(c: &mut Criterion) {
    c.bench_function("piped ruby", |b| {
        b.iter(|| {
            oursh_release!("{#!ruby; puts 1}");
        })
    });

    c.bench_function("piped node", |b| {
        b.iter(|| {
            oursh_release!("{#!node; console.log(1)}");
        })
    });

    c.bench_function("piped python", |b| {
        b.iter(|| {
            oursh_release!("{#!python; print(1)}");
        })
    });

    c.bench_function("piped sh", |b| {
        b.iter(|| {
            oursh_release!("{#!/bin/sh; echo 1}");
        })
    });
}

criterion_group!(benches, shebang_block_benchmark);
criterion_main!(benches);
