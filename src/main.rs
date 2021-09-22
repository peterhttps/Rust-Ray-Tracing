mod vector;

extern crate image;

use std::f32;
use nalgebra::{ArrayStorage, Matrix, U1, U3, Vector3};
use vector::Ray;

fn convert_bit_to_u8(value: f32) -> u8 {
  if value > 255.0 {
    return 255;
  }
  if value < 0.0 {
    return 0;
  }
  let max = 255.0;
  (max * value) as u8
}

fn ray_color(ray: Ray) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  let t = 0.5 * (ray.direction.y + 1.0);
  (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {

    // IMAGE
    let aspect_ratio: f32 = 16.0 / 9.0;
    let im_width = 800.0;
    let im_height = (im_width / aspect_ratio) as u32;
    let mut imgbuf = image::ImageBuffer::new(im_width as u32, im_height);

    // CAMERA
    let viewport_height: f32 = 2.0;
    let viewport_width = (viewport_height * aspect_ratio as f32) as f32; 
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let focal_length: f32 = 1.0;
    let origin = Vector3::new(0.0, 0.0, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
      let u = (i as f32 - 1.0) / (im_width as f32 - 1.0) as f32; 
      let v = (1.0 - (j as f32 - 1.0) / (im_height as f32 - 1.0)) as f32;
      let dir = lower_left_corner + u*horizontal + v*vertical - origin;
      let ray = Ray::new(origin, dir);

      let rayc = ray_color(ray);

      let r = convert_bit_to_u8(rayc.x);
      let g = convert_bit_to_u8(rayc.y);
      let b = convert_bit_to_u8(rayc.z);

      *pixel = image::Rgb([r, g, b]);
  }
    
    imgbuf.save("src/rendered/image1.png").unwrap();
    println!("Render done!");
}
