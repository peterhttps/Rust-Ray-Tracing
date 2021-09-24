mod vector;

extern crate image;

use std::f32;
use nalgebra::{ArrayStorage, Matrix, U1, U3, Vector3};
use vector::{HitRecord, Ray, Sphere, scene_list_hit};

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
  (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn ray_color(ray: Ray, scenelist: SceneList) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  let mut record = HitRecord::default();

  if scene_list_hit(scenelist, ray, 0.0,std::f32::INFINITY, &mut record) {
    // let p = ray_at(ray, t);
    // let normal = p - sphere.center;
    // let normal = normal as Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>;
    // let normal = unitvector(normal);
    
    // let ncolor = 0.5 * (normal + Vector3::new(1.0, 1.0, 1.0));
    // println!("{}", ncolor);

    let ncolor = 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0));

    return ncolor;
  }

  background_color(ray.direction)
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
    let floor = Sphere::new(Vector3::new(0.0, -big_radius - 1.0, -1.0), big_radius);

    let mut world = SceneList::new();
    world.push(s1);
    world.push(floor);

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
    
    imgbuf.save("src/rendered/image4.png").unwrap();
    println!("Render done!");
}
