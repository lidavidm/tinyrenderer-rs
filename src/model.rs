use nalgebra::Vec3;
use regex;

pub struct Model {
    pub vertices: Vec<Vec3<f32>>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Model {
    pub fn parse(text: &str) -> Model {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let face_re = regex::Regex::new(r"f (\d+)/\d+/\d+ (\d+)/\d+/\d+ (\d+)/\d+/\d+").unwrap();
        for line in text.lines() {
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split(" ").collect();
                vertices.push(Vec3 {
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
