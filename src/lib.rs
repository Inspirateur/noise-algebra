mod noise;
mod algebra;
mod sources;
pub use crate::noise::{Noise, NoiseSource};
pub use crate::sources::perlin;
pub use crate::sources::simd;
pub use crate::sources::fake_noise;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_domain_output() {
        let noise = !fake_noise(0.5) + fake_noise(0.5)*2f64 - fake_noise(0.5);
        assert_eq!(noise.domain(), -4f64..=4f64);
        let noise =  noise.normalize();
        assert_eq!(noise.domain(), -1f64..=1f64);
        let sample = noise.sample(0..1, 0..1, 0);
        assert_eq!(sample[0], 0f64);
    }
}
