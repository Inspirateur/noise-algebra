use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noise_algebra::NoiseSource;
const SIZE: i32 = 32;
const UP_TO: i32 = SIZE-1;

fn generate_const2d(mut n: NoiseSource<2>) {
    let cont = (n.constant(0.7) + n.constant(3.) * 0.3).normalize();
    let land = (&cont + n.constant(9.) * 0.1).normalize().threshold(0.35);
    let mount_mask = (n.constant(1.) + n.constant(2.)*0.3).normalize().threshold(0.2) * &land;
    let mount = (!(n.constant(0.8).powi(2)) + n.constant(1.5).powi(2)*0.4).normalize() * mount_mask;
    let _y =land*0.3 + mount*(1.-0.3);
}

fn generate_simplex2d(mut n: NoiseSource<2>) {
    let cont = (n.simplex(0.7) + n.simplex(3.) * 0.3).normalize();
    let land = (&cont + n.simplex(9.) * 0.1).normalize().threshold(0.35);
    let mount_mask = (n.simplex(1.) + n.simplex(2.)*0.3).normalize().threshold(0.2) * &land;
    let mount = (!(n.simplex(0.8).powi(2)) + n.simplex(1.5).powi(2)*0.4).normalize() * mount_mask;
    let _y = land*0.3 + mount*(1.-0.3);
}

fn generate_const3d(mut n: NoiseSource<3>) {
    let cont = (n.constant(0.7) + n.constant(3.) * 0.3).normalize();
    let land = (&cont + n.constant(9.) * 0.1).normalize().threshold(0.35);
    let mount_mask = (n.constant(1.) + n.constant(2.)*0.3).normalize().threshold(0.2) * &land;
    let mount = (!(n.constant(0.8).powi(2)) + n.constant(1.5).powi(2)*0.4).normalize() * mount_mask;
    let _y =land*0.3 + mount*(1.-0.3);
}

fn generate_simplex3d(mut n: NoiseSource<3>) {
    let cont = (n.simplex(0.7) + n.simplex(3.) * 0.3).normalize();
    let land = (&cont + n.simplex(9.) * 0.1).normalize().threshold(0.35);
    let mount_mask = (n.simplex(1.) + n.simplex(2.)*0.3).normalize().threshold(0.2) * &land;
    let mount = (!(n.simplex(0.8).powi(2)) + n.simplex(1.5).powi(2)*0.4).normalize() * mount_mask;
    let _y = land*0.3 + mount*(1.-0.3);
}

fn bench_const2d(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::<2>::new(ranges.clone(), 0, 1);
    c.bench_function(&format!("const-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_const2d(
        black_box(noise.clone())
    )));
}

fn bench_perlin2d(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::<2>::new(ranges.clone(), 0, 1);
    c.bench_function(&format!("simplex-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_simplex2d(
        black_box(noise.clone())
    )));
}

fn bench_const3d(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::<3>::new(ranges.clone(), 0, 1);
    c.bench_function(&format!("const-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_const3d(
        black_box(noise.clone())
    )));
}

fn bench_perlin3d(c: &mut Criterion) {
    // the ranges to sample
    let ranges = [0..=UP_TO, 0..=UP_TO, 0..=UP_TO];
    let noise = NoiseSource::<3>::new(ranges.clone(), 0, 1);
    c.bench_function(&format!("simplex-generate-{}^{}", SIZE, ranges.len()), |b| b.iter(|| generate_simplex3d(
        black_box(noise.clone())
    )));
}

criterion_group!(noise, bench_const2d, bench_perlin2d, bench_const3d,  bench_perlin3d);
criterion_main!(noise);