//! Top-levul documentation

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use image::{Luma, ImageLuma8};


/// Points plotted on canvas
pub struct Point {
    x: u32,
    y: u32,
}


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const ITERATIONS: u32 = 1_000_000;


pub fn main() {
    let (fg_color, bg_color) = (Luma([0u8]), Luma([255u8]));

    let mut img = image::ImageBuffer::from_fn(
        WIDTH,
        HEIGHT,
        |x, y| {
            if x == 0 && y == 0 { fg_color } else { bg_color }
        },
    );

    let points: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0},
        Point {x: 0, y: HEIGHT},
        Point {x: WIDTH, y: HEIGHT},
    ];
    let mut num: usize; 

    let mut p = Point {x: 350, y: 350};

    for _ in 0..ITERATIONS {
        num = rand::thread_rng().gen_range(0, 3);

        p.x = (p.x + points[num].x) / 2;
        p.y = (p.y + points[num].y) / 2;

        img.put_pixel(p.x, p.y, fg_color);
    }

    let ref mut out = File::create(&Path::new("sierpinski.png")).unwrap();
    let _ = ImageLuma8(img).save(out, image::PNG);
}
