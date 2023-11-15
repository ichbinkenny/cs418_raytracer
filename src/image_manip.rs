use std::collections::HashMap;

use image::{Rgba, RgbaImage, Rgb};
use crate::shapes::{RayShape, Point};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::vector3::Vector3;
use crate::sun::Sun;

const LINEAR_SRGB_CUTOFF: f64 = 0.0031308;

pub struct RaytraceImage {
    filename: String,
    image: RgbaImage,
    current_color: [f64; 4],
    shapes: Vec<Box<dyn RayShape>>,
    suns: Vec<Sun>,
    forward_direction: Vector3,
    up_direction: Vector3,
    right_direction: Vector3,
    ray_origin: Point,
    pixels: HashMap<(usize, usize), Rgba<u8>>,
    exposure_value: f64,
}

impl RaytraceImage {
    pub fn new(width: u32, height: u32, filename: &str) -> Self {
        // Fill our collisions with empty data.
        Self {
            filename: String::from(filename),
            image: RgbaImage::new(width, height),
            current_color: [1.0, 1.0, 1.0, 1.0],
            shapes: vec![],
            suns: vec![],
            forward_direction: Vector3::new(0.0, 0.0, -1.0),
            up_direction: Vector3::new(0.0, 1.0, 0.0).normalize(),
            right_direction: Vector3::new(1.0, 0.0, 0.0).normalize(),
            ray_origin: Point::new(0.0, 0.0, 0.0),
            pixels: HashMap::new(),
            exposure_value: 1.0,
        }
    }

    pub fn save(&self) {
        match self.image.save(&self.filename) {
            Ok(_) => {},
            Err(e) => panic!("Failed to save image as {}. Error: {}", self.filename, e),
        }
    }

    pub fn render(&mut self) {
        let max = (if self.image.width() > self.image.height() { self.image.width() } else { self.image.height() }) as f64;
        for x in 0..self.image.width() {
            for y in 0..self.image.height() {
                let mut smallest_distance = std::f64::INFINITY;
                let mut found_shape = false;

                for i in 0..self.shapes.len()  {
                    let s_x = (2.0 * x as f64 - self.image.width() as f64) / max;
                    let s_y = (self.image.height() as f64 - (2.0 * y as f64)) / max;
                    let ray = Ray::new(self.forward_direction, self.up_direction, self.right_direction, self.ray_origin, s_x, s_y);
                    // For each shape at this pixel,
                    // Get the distance to intersection
                    // Choose the smallest intersection to be the color
                    let shape = &self.shapes[i];
                    match shape.get_intersection(&ray) {
                        Some(data) => {
                            if !found_shape {
                                smallest_distance = data.0;
                                found_shape = true;
                                let mut color = [0.0, 0.0, 0.0, 1.0];
                                self.pixels.insert((x as usize, y as usize), Rgba([0, 0, 0, 255]));
                                for sun in &self.suns {
                                    let next_color = shape.get_linear_color(&self.shapes, &ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], next_color[3] + color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0] / self.suns.len() as f64), (color[1]  / self.suns.len() as f64), (color[2] / self.suns.len() as f64), color[3] / self.suns.len() as f64];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_sRGB(color, self.exposure_value));
                                
                            } else if data.0 < smallest_distance {
                                smallest_distance = data.0;
                                let mut color = [0.0, 0.0, 0.0, 1.0];
                                for sun in &self.suns {
                                    let next_color = shape.get_linear_color(&self.shapes, &ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], next_color[3] + color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0] / self.suns.len() as f64), (color[1]  / self.suns.len() as f64), (color[2] / self.suns.len() as f64), color[3] / self.suns.len() as f64];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_sRGB(color, self.exposure_value));
                            }
                        },
                        None => {},
                    }
                    
                }

                // Place closest shape color
                if found_shape {
                    let color = self.pixels[&(x as usize, y as usize)];
                    self.place_pixel_rgba(x, y, &color);
                }
            }
        }
    }

    pub fn place_pixel_rgba(&mut self, x: u32, y: u32, color: &Rgba<u8>) {
        let mut x_pos = x;
        let mut y_pos = y;
        if x > self.image.width() {
            x_pos = self.image.width();
        }
        if y > self.image.height() {
            y_pos = self.image.height();
        }
        let converted_color = Rgba([color[0].min(255), color[1].min(255), color[2].min(255), color[3].min(255)]);
        self.image.put_pixel(x_pos, y_pos, converted_color);
    }

    pub fn place_pixel_rgb(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        self.place_pixel_rgba(x, y, &Rgba([color.0[0], color.0[1], color.0[2], 255]));
    }

    pub fn add_sphere(&mut self, x_position: f64, y_position: f64, z_position: f64, radius: f64) {
        let sphere = Sphere::new(x_position, y_position, z_position, radius, self.current_color);
        self.shapes.push(Box::new(sphere));
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.current_color = [r, g, b, 1.0];
    }

    pub fn add_sun(&mut self, x_position: f64, y_position: f64, z_position: f64) {
        let sun = Sun::new(Vector3::new(x_position, y_position, z_position), self.current_color);
        self.suns.push(sun);
    }

    pub fn add_plane(&mut self, x: f64, y: f64, z: f64, d: f64) {
        let plane = Plane::new(x, y, z, d, self.current_color);
        self.shapes.push(Box::new(plane));
    }

    pub fn set_exposure(&mut self, exposure_value: f64) {
        self.exposure_value = exposure_value;
    }

    pub fn set_eye_origin(&mut self, new_origin: &Point) {
        self.ray_origin = *new_origin;
    }

    pub fn set_forward_direction(&mut self, new_direction: &Vector3) {
        self.forward_direction = *new_direction;
        self.right_direction = self.forward_direction.cross(self.up_direction).normalize();
        self.up_direction = self.right_direction.cross(self.forward_direction).normalize();
    }

    pub fn set_up_direction(&mut self, new_direction: &Vector3) {
        self.up_direction = (*new_direction).normalize();
        self.right_direction = self.forward_direction.cross(self.up_direction).normalize();
    }

    fn linear_color_to_sRGB(linear_color: [f64; 4], exposure_value: f64) -> Rgba<u8> {
        let mut output_color = [0.0, 0.0, 0.0, 0.0]; 
        if linear_color[0] > LINEAR_SRGB_CUTOFF {
            output_color[0] = (1.055 * (linear_color[0].powf(1.0 / 2.4)) - 0.055) * exposure_value;
        } else {
            output_color[0] = 12.92 * linear_color[0] * exposure_value;
        }
        if linear_color[1] > LINEAR_SRGB_CUTOFF {
            output_color[1] = (1.055 * (linear_color[1].powf(1.0 / 2.4)) - 0.055) * exposure_value;
        } else {
            output_color[1] = 12.92 * linear_color[1] * exposure_value;
        }
        if linear_color[2] > LINEAR_SRGB_CUTOFF {
            output_color[2] = (1.055 * (linear_color[2].powf(1.0 / 2.4)) - 0.055) * exposure_value;
        } else {
            output_color[2] = 12.92 * linear_color[2] * exposure_value;
        }
        let output_color_u8 = [(output_color[0] * 255.0) as u8, (output_color[1] * 255.0) as u8, (output_color[2] * 255.0) as u8, 255];
        // println!("Converted: {:?} to sRGB: {:x?}", linear_color, output_color_u8);
        Rgba(output_color_u8)
    } 

}
