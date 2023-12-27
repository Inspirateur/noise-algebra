use ndarray::Array2;

use crate::Signal;

impl Signal<Array2<f32>> {
    pub fn grad(self) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        // TODO: convolve with a sobel kernel
        todo!()
    }
}