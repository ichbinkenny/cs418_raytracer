use image::Rgba;

use crate::vector3::Vector3;

pub struct Sun {
    pub light_direction: Vector3,
    pub light_color: [f64; 4],
}

impl Sun {
    pub fn new(direction: Vector3, color: [f64; 4]) -> Self {
        Self {
            light_direction: direction,
            light_color: color,
        }
    }

    pub fn get_color(&self) -> [f64; 4] {
        self.light_color
    }
}