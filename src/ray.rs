use crate::shapes::Point;
use std::f64;

use crate::vector3::Vector3;

#[derive(Copy, Clone)]
pub enum RayLensType {
    Standard,
    Fisheye,
    Panorama,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
    pub lens_type: RayLensType,
    pub generation: u32,
    pub max_bounces: u32,
}

impl Ray {
    pub fn new(forward_dir: Vector3, up_dir: Vector3, right_dir: Vector3, origin: Point, s_x: f64, s_y: f64, lens_type: RayLensType, max_bounces: u32) -> Self {
        let ray_direction = match lens_type {
            RayLensType::Standard => forward_dir + (s_x * right_dir) + (s_y * up_dir),
            RayLensType::Fisheye => {
                let new_f = (1.0 - s_x.powi(2) - s_y.powi(2)).sqrt() * forward_dir;
                new_f + (s_x * right_dir) + (s_y * up_dir)
            },
            RayLensType::Panorama => { 
                (s_y * up_dir) + (s_x.cos() * forward_dir) + (s_x.sin() * right_dir)
             },
        };
        Self {
            origin,
            direction: ray_direction.normalize(),
            lens_type,
            generation: 0,
            max_bounces,
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
