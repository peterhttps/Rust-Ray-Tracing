extern crate image;


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

fn main() {
    let im_width = 800;
    let im_height = 600;

    let mut imgbuf = image::ImageBuffer::new(im_width, im_height);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
      // let r = (0.3 * x as f32) as u8;
      let r = ((i as f32 - 1.0) / (im_width as f32 - 1.0)) as f32;
      let g = (1.0 - (j as f32 - 1.0) / (im_height as f32 - 1.0)) as f32;
      let b = 0.25 as f32;
      
      let r = convert_bit_to_u8(r);
      let g = convert_bit_to_u8(g);
      let b = convert_bit_to_u8(b);

      *pixel = image::Rgb([r, g, b]);
  }
    
    imgbuf.save("src/rendered/image0.png").unwrap();
    println!("Render done!");
}
