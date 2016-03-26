#![feature(associated_consts)]
extern crate image;

use std::io;
use std::mem;

struct Image{
    buf: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        Image {
            buf: vec![0; width * height * 4],
            width: width,
            height: height,
        }
    }

    fn set(&mut self, x: usize, y: usize, color: &Color) {
        let y = self.height - y;
        let offset = y * 4 * self.width + x * 4;
        self.buf[offset] = color.r;
        self.buf[offset + 1] = color.g;
        self.buf[offset + 2] = color.b;
        self.buf[offset + 3] = color.a;
    }

    fn save(&self, path: &str) -> Result<(), io::Error> {
        image::save_buffer(path, &self.buf, self.width as u32, self.height as u32, image::RGBA(8))
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
}

fn line(image: &mut Image, color: &Color, x0: isize, y0: isize, x1: isize, y1: isize) {
    let steep = (x0-x1).abs() < (y0-y1).abs();
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;
    if steep {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    for x in x0..x1 + 1 {
        let t = (x - x0) as f32 / (x1 as f32 - x0 as f32);
        let y = ((y0 as f32) * (1.0 - t) + (y1 as f32) * t) as isize;

        if steep {
            image.set(y as usize, x as usize, color);
        }
        else {
            image.set(x as usize, y as usize, color);
        }
    }
}

fn main() {
    let mut image = Image::new(100, 100);
    line(&mut image, &Color::RED, 13, 20, 80, 40);
    line(&mut image, &Color::RED, 20, 13, 40, 80);
    image.save("test2.png");
}
