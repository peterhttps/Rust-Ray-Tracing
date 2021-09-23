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

pub fn ray_at(ray: Ray, t: f32) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  ray.origin + t * ray.direction
}

pub fn hit(sphere: Sphere, ray: Ray) -> f32 {
  let a = ray.direction.norm_squared();
  let oc = ray.origin - sphere.center;
  let halfb = ray.direction.dot(&oc);
  let c = oc.norm_squared() - f32::powf(sphere.radius, 2.0);
  let discriminant = halfb*halfb - a*c;

  if discriminant < 0.0 {
    return -1.0;
  } else {
    let t = (-halfb - discriminant.sqrt()) / (a);  
    return t;
  }
}