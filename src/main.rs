mod vector;

extern crate image;

use std::{f32, time::Instant};
use nalgebra::{ArrayStorage, Matrix, U1, U3, Vector3};
use vector::{HitRecord, Ray, Sphere, reflect, scene_list_hit, unitvector};

use crate::vector::SceneList;

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


fn background_color(dir: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  let t = 0.5 * (dir.y + 1.0);
  (1.0 - t) * Vector3::new(0.7, 0.8, 0.9) + t * Vector3::new(0.05, 0.05, 0.2)
}

fn ray_color(ray: Ray, scenelist: SceneList) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  let mut record = HitRecord::default();

  let mut orgn = ray.origin;
  let mut dir = ray.direction;
  while scene_list_hit(scenelist.clone(), Ray::new(orgn, dir), 0.01,std::f32::INFINITY, &mut record) {

    dir = unitvector(reflect(dir, record.normal));
    orgn = record.p;

    // let ncolor = 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0));

    // return ncolor;
  }

  background_color(dir)
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

    let big_radius: f32 = 1000.0;
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vector3::new(-1.5, 0.0, -2.0), 0.5);
    let s3 = Sphere::new(Vector3::new(0.5, 1.25, -1.5), 0.5);
    let s4 = Sphere::new(Vector3::new(1.5, 0.5, -2.0), 0.5);
    let floor = Sphere::new(Vector3::new(0.0, -big_radius - 0.5, -1.0), big_radius);

    let mut world = SceneList::new();
    world.push(s1);
    world.push(s2);
    world.push(s3);
    world.push(s4);
    world.push(floor);

    let start = Instant::now();
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
      let u = (i as f32 - 1.0) / (im_width as f32 - 1.0) as f32; 
      let v = (1.0 - (j as f32 - 1.0) / (im_height as f32 - 1.0)) as f32;
      let dir = lower_left_corner + u*horizontal + v*vertical - origin;
      let ray = Ray::new(origin, dir);

      let rayc = ray_color(ray, world.clone());

      let r = convert_bit_to_u8(rayc.x);
      let g = convert_bit_to_u8(rayc.y);
      let b = convert_bit_to_u8(rayc.z);

      *pixel = image::Rgb([r, g, b]);
    }
    
    imgbuf.save("src/rendered/image7.png").unwrap();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Render done!");
}
