mod noise;
mod algebra;
mod perlin;
use crate::noise::Noise;
use crate::perlin::perlin;

fn main() {
    let noise = 1.0 + perlin(2.0) + perlin(33.0) + 1.0;
    let terrain = |f: f64| perlin(3.0 * f) + 3.0 * f;
    println!("{}", noise.sample(0.0, 0.0, 0));
    println!("{}", noise.sample(0.1, 0.0, 0));
    println!("{}", noise.sample(0.2, 0.0, 0));
    println!("{}", noise.sample(0.3, 0.0, 0));
    println!("{}", terrain(4.0).sample(0.3, 0.0, 1));
}
