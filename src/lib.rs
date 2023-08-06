mod noise;
mod algebra;
mod perlin;
mod simplex;
pub use crate::noise::{Noise, NoiseSource};
pub use crate::perlin::perlin;
pub use crate::simplex::simplex;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_simd() {
        let noise = perlin(1.);
        println!("{}", noise.sample(0..8, 0..8, 1));
    }
}
