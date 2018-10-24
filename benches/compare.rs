#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path="../tests/common/mod.rs"]
mod common;

fn compare_benchmark(c: &mut Criterion) {
    c.bench_function("oursh_script", |b| {
        b.iter(|| {
            oursh_release!(> "scripts/hello_world.sh");
        })
    });

    c.bench_function("sh_script", |b| {
        b.iter(|| {
            shell!(> "/bin/sh", "scripts/hello_world.sh")
        })
    });

    c.bench_function("zsh_script", |b| {
        b.iter(|| {
            shell!(> "/usr/bin/zsh", "scripts/hello_world.sh")
        })
    });

    c.bench_function("fish_script", |b| {
        b.iter(|| {
            shell!(> "/usr/bin/fish", "scripts/hello_world.sh")
        })
    });
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
