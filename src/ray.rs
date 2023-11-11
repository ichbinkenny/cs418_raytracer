use crate::shapes::Point;
use std::f32;

use crate::vector3::Vector3;

pub struct Ray {
    origin: Point,
    direction: Vector3,
}

impl Ray {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector3::new(x as f64, y as f64, 0.0).normalize(),
        }
    }
    pub fn set_origin(&mut self, point: Point) {
        self.origin = point;
    }
    pub fn get_origin(&self) -> Point {
        self.origin
    }
    pub fn set_direction(&mut self, direction: Vector3) {
        self.direction = direction;
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }

}
