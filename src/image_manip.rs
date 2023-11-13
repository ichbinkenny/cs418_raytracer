use std::borrow::BorrowMut;
use std::collections::HashMap;

use image::{Rgba, RgbaImage, Rgb};
use crate::shapes::RayShape;
use crate::ray::Ray;
use crate::sphere::Sphere;
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
    pixels: HashMap<(usize, usize), Rgba<u8>>,
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
            pixels: HashMap::new(),
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
                    let ray = Ray::new(self.forward_direction, self.up_direction, self.right_direction, s_x, s_y);
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
                                    let next_color = shape.get_linear_color(&ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0] / self.suns.len() as f64), (color[1]  / self.suns.len() as f64), (color[2] / self.suns.len() as f64), color[3]];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_sRGB(color));
                                
                            } else if data.0 < smallest_distance {
                                smallest_distance = data.0;
                                let mut color = [0.0, 0.0, 0.0, 255.0];
                                for sun in &self.suns {
                                    let next_color = shape.get_linear_color(&ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0] / self.suns.len() as f64), (color[1] / self.suns.len() as f64), (color[2] / self.suns.len() as f64), color[3]];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_sRGB(color));
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
        let converted_color = Rgba([color[0], color[1], color[2], color[3]]);
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

    fn linear_color_to_sRGB(linear_color: [f64; 4]) -> Rgba<u8> {
        let mut output_color = [0.0, 0.0, 0.0, 255.0]; 
        if linear_color[0] > LINEAR_SRGB_CUTOFF {
            output_color[0] = 1.055 * (linear_color[0].powf(1.0 / 2.4)) - 0.055;
        } else {
            output_color[0] = 12.92 * linear_color[0];
        }
        if linear_color[1] > LINEAR_SRGB_CUTOFF {
            output_color[1] = 1.055 * (linear_color[1].powf(1.0 / 2.4)) - 0.055;
        } else {
            output_color[1] = 12.92 * linear_color[1];
        }
        if linear_color[2] > LINEAR_SRGB_CUTOFF {
            output_color[2] = 1.055 * (linear_color[2].powf(1.0 / 2.4)) - 0.055;
        } else {
            output_color[2] = 12.92 * linear_color[2];
        }
        let output_color_u8 = [(output_color[0] * 255.0) as u8, (output_color[1] * 255.0) as u8, (output_color[2] * 255.0) as u8, output_color[3] as u8];
        println!("Converted: {:?} to sRGB: {:x?}", linear_color, output_color_u8);
        Rgba(output_color_u8)
    } 

}
