pub mod vec;

use std::fs;
use std::io::Write;
use std::path::Path;
use vec::Vec3;

#[derive(Debug)]
pub struct File {
    pub handle: std::fs::File,
}

impl File {
    pub fn new(path: &str) -> File {
        let path = Path::new(path);
        let handle = match fs::File::create(&path) {
            Err(err) => panic!("error {}", err),
            Ok(file) => file,
        };

        File { handle: handle }
    }

    pub fn write(&self, line: String) {
        let mut file = &self.handle;

        match file.write_all(line.as_bytes()) {
            Err(err) => panic!("error {}", err),
            Ok(_) => {}
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub direction: Vec3,
    pub origin: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.mul(t))
    }

    pub fn color(&self) -> Vec3 {
        let dir = self.direction.unit();
        let t = 0.5 * dir.y + 1.0;

        // linear interpolation between white (1.0, 1.0, 1.0) and blue(0.5, 0.7, 1.0)
        let color1 = Vec3::new(1.0, 1.0, 1.0);
        let color2 = Vec3::new(0.5, 0.7, 1.0);

        // linear interpolation (lerp)
        // blendedValue = (1−t) ⋅ startValue + t ⋅ endValue
        color1.mul(1.0 - t).add(&color2.mul(t))
    }
}
