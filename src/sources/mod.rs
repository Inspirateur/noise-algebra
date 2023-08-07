mod perlin;
mod simplex;
mod fake_noise;
pub use perlin::perlin;
pub use simplex::simplex;
pub use fake_noise::fake_noise;
pub(crate) const C: f64 = 3141.592653589793238462643383279502884197;
