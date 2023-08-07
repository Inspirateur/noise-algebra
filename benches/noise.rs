use std::ops::Range;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noise_algebra::{NoiseSource, Noise, perlin, simd::simplex, fake_noise};
const SIZE: i32 = 32;

fn generate<X: Noise>(noise: NoiseSource<X>, x_range: Range<i32>, y_range: Range<i32>) {
    let _ = noise.sample(x_range, y_range, 42);
}

fn const_bench(c: &mut Criterion) {
    // generate coordinates to sample
    let cont = (fake_noise(0.7) + fake_noise(3.) * 0.3).normalize();
    let land = (cont.clone() + fake_noise(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (fake_noise(1.) + fake_noise(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(fake_noise(0.8).powi(2)) + fake_noise(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
    c.bench_function(&format!("const-generate-{}^2", SIZE), |b| b.iter(|| generate(
        black_box(y.clone()), black_box(0..SIZE), black_box(0..SIZE)
    )));
}

fn perlin_bench(c: &mut Criterion) {
    // generate coordinates to sample
    let cont = (perlin(0.7) + perlin(3.) * 0.3).normalize();
    let land = (cont.clone() + perlin(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (perlin(1.) + perlin(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(perlin(0.8).powi(2)) + perlin(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
    c.bench_function(&format!("perlin-generate-{}^2", SIZE), |b| b.iter(|| generate(
        black_box(y.clone()), black_box(0..SIZE), black_box(0..SIZE)
    )));
}

fn simd_bench(c: &mut Criterion) {
    // generate coordinates to sample
    let cont = (simplex(0.7) + simplex(3.) * 0.3).normalize();
    let land = (cont.clone() + simplex(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (simplex(1.) + simplex(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(simplex(0.8).powi(2)) + simplex(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
    c.bench_function(&format!("simd-generate-{}^2", SIZE), |b| b.iter(|| generate(
        black_box(y.clone()), black_box(0..SIZE), black_box(0..SIZE)
    )));
}

criterion_group!(noise, const_bench, perlin_bench, simd_bench);
criterion_main!(noise);