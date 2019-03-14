extern crate nalgebra as na;

extern crate image;

extern crate alga;

extern crate rand;

use rand::Rng;
use image::GenericImage;
use image::Pixel;
use image::{DynamicImage, Rgba};

use std::fs::File;

use na::{Point3, Vector3, Matrix4};

pub mod scene;
pub mod model;
pub mod util;
pub mod core;

fn render() -> DynamicImage {
    let mut rng = rand::thread_rng();
    let mut image = DynamicImage::new_rgb8(800, 600);
    for x in 0..800 {
        for y in 0..600 {
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();
            let black = Rgba::from_channels(r, g, b, 255);
            image.put_pixel(x, y, black)
        }
    }
    image
}

fn main() {
    println!("Learning Raytracer");
    let img = render();
    let mut out = File::create("out.png").unwrap();
    img.write_to(&mut out, image::PNG).expect("Saving image failed");
    println!("Done. Exiting...");
}
