use crate::shapes::Point;
use std::f64;

use crate::vector3::Vector3;

pub struct Ray {
    origin: Point,
    direction: Vector3,
}

impl Ray {
    pub fn new(forward_dir: Vector3, up_dir: Vector3, right_dir: Vector3, s_x: f64, s_y: f64) -> Self {
        let ray_direction = forward_dir + (s_x * right_dir) + (s_y * up_dir);
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: ray_direction.normalize(),
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
