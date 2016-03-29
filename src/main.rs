#![feature(associated_consts)]
extern crate image;
extern crate nalgebra;
extern crate rand;
extern crate regex;

mod model;

use std::io;
use std::mem;

use nalgebra::{cross, Pnt2, Vec2, Vec3};

use model::Model;

struct Image {
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

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        Color::new(
            rng.gen(),
            rng.gen(),
            rng.gen(),
            255,
        )
    }
}

fn line(image: &mut Image, color: &Color, x0: isize, y0: isize, x1: isize, y1: isize) {
    if x0 == x1 {
        let (y0, y1) = if y0 > y1 {
            (y1, y0)
        }
        else {
            (y0, y1)
        };
        for y in y0..y1 + 1 {
            image.set(x0 as usize, y as usize, color);
        }
        return;
    }

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

fn to_barycentric(point: Pnt2<i32>, triangle: &[Pnt2<i32>; 3]) -> Vec3<f32> {
    let point: Pnt2<f32> = nalgebra::cast(point);
    let triangle: &[Pnt2<f32>; 3] = &[nalgebra::cast(triangle[0]), nalgebra::cast(triangle[1]), nalgebra::cast(triangle[2])];
    let u = cross(&Vec3::new(triangle[2].x - triangle[0].x, triangle[1].x - triangle[0].x, triangle[0].x - point.x),
                  &Vec3::new(triangle[2].y - triangle[0].y, triangle[1].y - triangle[0].y, triangle[0].y - point.y));
    if u.z.abs() < 1. {
        // Degenerate triangle; return a result with negative
        // coordinates so that we discard the point in triangle
        // rasterization
        return Vec3::new(-1., 1., 1.);
    }
    else {
        return Vec3::new(1. - (u.x + u.y) / u.z,
                         u.y / u.z,
                         u.x / u.z);
    }
}

fn triangle(image: &mut Image, color: &Color, triangle: &[Pnt2<i32>; 3]) {
    let mut bbox_min = Pnt2::new((image.width - 1) as i32, (image.height - 1) as i32);
    let mut bbox_max = Pnt2::new(0, 0);
    let bbox_clamp = Pnt2::new((image.width - 1) as i32, (image.height - 1) as i32);

    for point in triangle {
        for coord in 0..2 {
            bbox_min[coord] = std::cmp::max(
                0,
                std::cmp::min(bbox_min[coord], point[coord]));
            bbox_max[coord] = std::cmp::min(
                bbox_clamp[coord],
                std::cmp::max(bbox_max[coord], point[coord]));
        }
    }

    for x in bbox_min.x..bbox_max.x + 1 {
        for y in bbox_min.y..bbox_max.y + 1 {
            let p = Pnt2::new(x, y);
            let bc_screen = to_barycentric(p, triangle);
            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            }
            image.set(p.x as usize, p.y as usize, color);
        }
    }
}

fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    let path = Path::new("african_head.obj");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let width = 1000;
    let height = 1000;
    let mut image = Image::new(width, height);
    let convert_coord = |v: &Vec3<f32>| {
        Pnt2::new(1 + ((v.x + 1.0) * ((width - 1) as f32) / 2.0) as i32,
                  1 + ((v.y + 1.0) * ((height - 1) as f32) / 2.0) as i32)
    };
    let model = Model::parse(&s);
    {
        for face in model.faces {
            triangle(&mut image, &rand::random(), &[
                convert_coord(&model.vertices[face.0]),
                convert_coord(&model.vertices[face.1]),
                convert_coord(&model.vertices[face.2]),
            ]);
        }
    }

    image.save("test3.png");
}
