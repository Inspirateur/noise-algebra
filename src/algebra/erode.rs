use ndarray::Array2;

use crate::NoiseSource;

pub trait ErosionTrait {
    fn erode(self, n: NoiseSource<2>, strength: f32, octaves: usize) -> (Array2<f32>, Array2<f32>, Array2<f32>);
}

// data is (height, G_x, G_y) like in the shadertoy link
impl ErosionTrait for (Array2<f32>, Array2<f32>, Array2<f32>) {
    fn erode(self, n: NoiseSource<2>, strength: f32, octave: usize) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        // TODO: see: https://www.shadertoy.com/view/7ljcRW
        todo!()
    }
}