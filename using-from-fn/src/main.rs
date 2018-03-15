extern crate nalgebra;
extern crate image;

use nalgebra::DMatrix;
use image::Pixel;
use std::fs::File;
use std::path::Path;

fn main() {
    let mat: DMatrix<u32> = DMatrix::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{:?}", mat);

    let buffer = image::ImageBuffer::from_fn(512u32, 512u32, |x: u32, y: u32| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let mut out = File::create(&Path::new("out_pattern.png")).unwrap();
    img.save(&mut out, image::PNG);
}
