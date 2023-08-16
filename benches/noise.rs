use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noise_algebra::NoiseSource;
const SIZE: i32 = 32;
const UP_TO: i32 = SIZE-1;

fn generate_const<const D: usize>(n: NoiseSource<D>) {
    let cont = (n.constant(0.7) + n.constant(3.) * 0.3).normalize();
    let land = (cont.clone() + n.constant(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (n.constant(1.) + n.constant(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(n.constant(0.8).powi(2)) + n.constant(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
}

fn generate_simplex<const D: usize>(n: NoiseSource<D>) {
    let cont = (n.simplex(0.7) + n.simplex(3.) * 0.3).normalize();
    let land = (cont.clone() + n.simplex(9.) * 0.1).normalize().mask(0.35);
    let mount_mask = (n.simplex(1.) + n.simplex(2.)*0.3).normalize().mask(0.2)*land.clone();
    let mount = (!(n.simplex(0.8).powi(2)) + n.simplex(1.5).powi(2)*0.4).normalize() * mount_mask;
    let y = 0.009 + land*0.3 + mount*(1.-0.3);
}

fn const_bench(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::new(ranges, 0, 1);
    c.bench_function(&format!("const-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_const(
        black_box(noise)
    )));
}

fn perlin_bench(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::new(ranges, 0, 1);
    c.bench_function(&format!("simplex-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_simplex(
        black_box(noise)
    )));
}


criterion_group!(noise, const_bench, perlin_bench);
criterion_main!(noise);