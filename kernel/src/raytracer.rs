//! This module is based off another great blogpost, ["Ray Tracing in One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage).
// TODO: It'd be nice to get rid of these excessive clones, think about changes that'd be nice in `fixed-vectors 3.3.x`

use fixed_vectors::{Vector2, Vector3};
use crate::writer::FrameBufferWriter;


pub struct Raytracer<'a> {
    writer: &'a mut FrameBufferWriter<'static>,
    resolution: Vector2<usize>,
}


impl<'a> Raytracer<'a> {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

    pub fn new(resolution: Vector2<usize>, writer: &'a mut FrameBufferWriter<'static>) -> Self {
        Self {
            resolution,
            writer
        }
    }

    pub fn run(&mut self) {
        // Camera
        let focal_length = 1.0;
        let viewport_resolution = Vector2::new(
            Self::ASPECT_RATIO * 2.0,
            2.0
        );

        let horizontal = Vector3::new(viewport_resolution.x, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_resolution.y, 0.0);
        let origin = Vector3::from_value(0.0);

        let lower_left_corner = origin.clone()
            - (horizontal.clone() / 2.0) - (vertical.clone() / 2.0)
            - Vector3::new(0.0, 0.0, focal_length);


        // Easiest way for us to call `writer.write_pixel` from top to down while keeping proper color order.
        let mut y_decrement = self.resolution.y - 1;

        for y in 0..self.resolution.y {
            for x in 0..self.resolution.x {
                let (u, v) = (
                    x as f64 / (self.resolution.x - 1) as f64,
                    y_decrement as f64 / (self.resolution.y - 1) as f64,
                );

                let ray = Ray {
                    origin: origin.clone(),
                    direction: lower_left_corner.clone() + (horizontal.clone() * u) + (vertical.clone() * v) - origin.clone()
                };

                let color = ray.color()
                    .map(|f| (255.999 * f) as u8);

                let position = Vector2::new(x, y);

                self.writer.write_pixel(
                    position,
                    color
                );
            }

            y_decrement -= 1;
        }
    }
}


struct Ray {
    direction: Vector3<f64>,
    origin: Vector3<f64>,
}


impl Ray {
    pub fn hit_sphere(&self, center: Vector3<f64>, radius: f64) -> f64 {
        let oc = self.origin.clone() - center;

        let a = self.direction.dot(&self.direction);
        let b = oc.dot(&self.direction) * 2.0;
        let c = oc.dot(&oc) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;
        
        match discriminant < 0.0 {
            false => (-b - libm::sqrt(discriminant)) / (2.0 * a),
            true => -1.0,
        }
    }
    
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin.clone() + (self.direction.clone() * t)
    }

    pub fn color(&self) -> Vector3<f64> {
        let t = self.hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5);

        if t > 0.0 {
            let n = unit_vector(self.at(t) - Vector3::new(0.0, 0.0, -1.0));
            return (n + 1.0) * 0.5;
        }

        let direction = unit_vector(self.direction.clone());
        let t = (direction.y + 1.0) * 0.5;

        (Vector3::from_value(1.0) * (1.0 - t)) + (Vector3::new(0.5, 0.7, 1.0) * t)
    }
}


fn unit_vector(v: Vector3<f64>) -> Vector3<f64> {
    let length = v.length();
    v / length
}