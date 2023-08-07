use std::ops::Range;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noise_algebra::{NoiseSource, Noise, perlin, simplex};
const SIZE: i32 = 32;

fn generate<X: Noise>(noise: NoiseSource<X>, x_range: Range<i32>, y_range: Range<i32>) {
    let _ = noise.sample(x_range, y_range, 42);
}

fn gen_bench(c: &mut Criterion) {
    // generate coordinates to sample
    let cont = (simplex(0.7) + simplex(3.) * 0.3).normalize();
    let land = (cont.clone() + simplex(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (simplex(1.) + simplex(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(simplex(0.8).powi(2)) + simplex(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
    c.bench_function(&format!("generate-{}^2", SIZE), |b| b.iter(|| generate(
        black_box(y.clone()), black_box(0..SIZE), black_box(0..SIZE)
    )));
}

criterion_group!(noise, gen_bench);
criterion_main!(noise);