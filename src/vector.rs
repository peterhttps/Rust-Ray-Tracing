use nalgebra::{ArrayStorage, Matrix, U1, U3};

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Sphere {
  pub center: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, radius: f32) -> Sphere {
    Sphere {
      center,
      radius
    }
  }
}

pub fn hit(sphere: Sphere, ray: Ray) -> bool {
  let a = ray.direction.norm_squared();
  let oc = sphere.center - ray.origin;
  let b = 2.0 * ray.direction.dot(&oc);
  let c = oc.norm_squared() - f32::powf(sphere.radius, 2.0);
  let discriminant = b*b - 4.0*a*c;

  if discriminant <= 0.0 {
    return false;
  }
  true
}