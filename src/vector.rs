use nalgebra::{ArrayStorage, Matrix, U1, U3};


pub struct Ray {
  pub origin: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
  pub direction: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
}

impl Ray {
  pub fn new(origin: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, direction: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) -> Ray {
    Ray {
      origin,
      direction
    }
  }
}