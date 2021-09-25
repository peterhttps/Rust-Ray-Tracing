mod vector;

extern crate image;

use indicatif::ProgressBar;
use rand::Rng;
use std::{f32, time::Instant};
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

  while scene_list_hit(scenelist.clone(), ray, 0.01,std::f32::INFINITY, &mut record) {

    let color = 0.5 * (record.normal + Vector3::new(1.0 ,1.0, 1.0)); 


    return color;
    // dir = unitvector(reflect(dir, record.normal));
    // orgn = record.p;

    // let ncolor = 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0));

    // return ncolor;
  }

  background_color(ray.direction)
}

fn main() {

    // IMAGE
    let aspect_ratio: f32 = 16.0 / 9.0;
    let im_width = 800.0;
    let im_height: u32 = (im_width / aspect_ratio) as u32;
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
    let floor = Sphere::new(Vector3::new(0.0, -big_radius - 0.5, -1.0), big_radius);

    let mut world = SceneList::new();
    world.push(s1);
    // world.push(s2);
    // world.push(s3);
    // world.push(s4);
    world.push(floor);

    println!("\nRendering time...");

    let samples_per_pixel = 100;
    let mut rng = rand::thread_rng();

    let bar = ProgressBar::new(im_height as u64);
    let start = Instant::now();

    for j in 0..im_height as u32 {
      for i in 0..im_width as u32 {
        let mut pixelcolor = Vector3::new(0.0, 0.0, 0.0);
        for n in 0..samples_per_pixel {
          let u = (i as f32 - 1.0 + rng.gen::<f32>()) / (im_width as f32 - 1.0) as f32; 
          let v = (1.0 - (j as f32 - 1.0 + rng.gen::<f32>()) / (im_height as f32 - 1.0)) as f32;
          let dir = lower_left_corner + u*horizontal + v*vertical - origin;
          let ray = Ray::new(origin, dir);
    
          pixelcolor += ray_color(ray, world.clone());
        }

        pixelcolor = pixelcolor / samples_per_pixel as f32;

        let pixel = imgbuf.get_pixel_mut(i, j);

        let r = convert_bit_to_u8(pixelcolor.x);
        let g = convert_bit_to_u8(pixelcolor.y);
        let b = convert_bit_to_u8(pixelcolor.z);
        *pixel = image::Rgb([r, g, b]);
      }
      bar.inc(1);
    }

    imgbuf.save("src/rendered/image7.png").unwrap();
    let duration = start.elapsed();
    println!("Render done!");
    println!("Time elapsed: {:.2?}", duration);
    println!("Render done!");
}
