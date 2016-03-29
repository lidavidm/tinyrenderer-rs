#![feature(associated_consts)]
extern crate image;
extern crate regex;

use std::io;
use std::mem;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

struct Model {
    vertices: Vec<Vertex>,
    faces: Vec<(usize, usize, usize)>,
}

impl Model {
    fn parse(text: &str) -> Model {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let face_re = regex::Regex::new(r"f (\d+)/\d+/\d+ (\d+)/\d+/\d+ (\d+)/\d+/\d+").unwrap();
        for line in text.lines() {
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split(" ").collect();
                vertices.push(Vertex {
                    x: parts[1].parse().unwrap(),
                    y: parts[2].parse().unwrap(),
                    z: parts[3].parse().unwrap(),
                });
            }
            else if line.starts_with("f ") {
                for cap in face_re.captures_iter(line) {
                    let (f0, f1, f2): (usize, usize, usize) =
                        (cap.at(1).unwrap().parse().unwrap(),
                         cap.at(2).unwrap().parse().unwrap(),
                         cap.at(3).unwrap().parse().unwrap());
                    faces.push((f0 - 1, f1 - 1, f2 - 1));
                }
            }
        }

        Model {
            vertices: vertices,
            faces: faces,
        }
    }
}

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

fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    let path = Path::new("african_head.obj");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut image = Image::new(1000, 1000);
    let model = Model::parse(&s);
    {
        let mut draw_line = |v0: &Vertex, v1: &Vertex| {
            let x0 = 1 + ((v0.x + 1.0) * (image.width - 1) as f32 / 2.0) as isize;
            let y0 = 1 + ((v0.y + 1.0) * (image.height - 1) as f32 / 2.0) as isize;
            let x1 = 1 + ((v1.x + 1.0) * (image.width - 1) as f32 / 2.0) as isize;
            let y1 = 1 + ((v1.y + 1.0) * (image.height - 1) as f32 / 2.0) as isize;
            line(&mut image, &Color::RED, x0, y0, x1, y1);

        };
        for face in model.faces {
            draw_line(&model.vertices[face.0], &model.vertices[face.1]);
            draw_line(&model.vertices[face.1], &model.vertices[face.2]);
            draw_line(&model.vertices[face.2], &model.vertices[face.0]);
        }
    }

    image.save("test3.png");
}
