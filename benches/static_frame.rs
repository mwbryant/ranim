#![feature(test)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate ranim;

fn short_static() {
    let obj = ranim::mobject::Mobject::Rectangle {
        x: 250.,
        y: 250.,
        w: 100.,
        h: 100.,
        color: String::from("blue"),
    };
    let mut scene = ranim::scene::Scene::new(500, 500).appear(&obj).wait(1.0);
    scene.render(std::io::sink()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("test");
    group.measurement_time(std::time::Duration::new(20, 0));
    group.bench_function("static frame", |b| b.iter(|| short_static()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
