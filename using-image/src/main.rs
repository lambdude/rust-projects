extern crate image;
extern crate rand;

use image::{FilterType, GenericImage, Pixel};
use std::fs::File;
use rand::distributions::{IndependentSample, Normal};

fn main() {
    let img = image::open("data/lena.png").expect("Opening image failed");
    let flipped = img.fliph();
    let mut flipped_out = File::create("flipped_lena.png").unwrap();
    flipped.save(&mut flipped_out, image::PNG).expect("Saving image failed");

    // Edge detection
    let kernel = [-1.0f32, -1.0, -1.0,
                    -1.0, 8.0, -1.0,
                    -1.0, -1.0, -1.0];
    let edgy = img.filter3x3(&kernel);
    let mut edgy_out = File::create("edgy_lena.png").unwrap();
    edgy.save(&mut edgy_out, image::PNG).expect("Saving image failed");

    // Manipulating pixels
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let normal = Normal::new(15.0, 15.0);
    let mut noisy = img.brighten(-25);
    for x in 0..(width) {
        for y in 0..(height) {
            let offset = normal.ind_sample(&mut rng) as u8;
            let px = img.get_pixel(x, y).map(|v| if v <= 255 - offset { v + offset } else { 255 });
            noisy.put_pixel(x, y, px);
        }
    }
    let mut noisy_out = File::create("noisy_lena.png").unwrap();
    noisy.save(&mut noisy_out, image::PNG).expect("Saving image failed");

    let thumbnail = img.resize(120, 120, FilterType::Lanczos3);
    let mut thumb_out = File::create("thumb_lena.png").unwrap();
    thumbnail.save(&mut thumb_out, image::PNG).expect("Saving image failed");

    // --release for 8-10x speed increase
}
