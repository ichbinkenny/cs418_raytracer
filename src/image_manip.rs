use image::{Rgba, RgbaImage, Rgb};
use crate::shapes::{RayShape, Sphere};
use crate::ray::Ray;

pub struct RaytraceImage {
    filename: String,
    image: RgbaImage,
    current_color: Rgba<u8>,
    shapes: Vec<Box<dyn RayShape>>,
}

impl RaytraceImage {
    pub fn new(width: u32, height: u32, filename: &str) -> Self {
        Self {
            filename: String::from(filename),
            image: RgbaImage::new(width, height),
            current_color: Rgba([255, 255, 255, 255]),
            shapes: vec![],
        }
    }

    pub fn save(&self) {
        match self.image.save(&self.filename) {
            Ok(_) => {},
            Err(e) => panic!("Failed to save image as {}. Error: {}", self.filename, e),
        }
    }

    pub fn render(&mut self) {
        for x in 0..self.image.width() {
            for y in 0..self.image.height() {
                for shape_number in 0..self.shapes.len()  {
                    let ray = Ray::new(x, y);
                    let shape = &self.shapes[shape_number];
                    match shape.get_intersection(&ray) {
                        Ok((data)) => {
                            println!("{}: Shape intersects at: {:?}", y + (x * self.image.width()), data);
                            self.place_pixel_rgba(x, y, &shape.get_color());
                        },
                        Err(_) => {
                        },
                    }
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
        self.image.put_pixel(x_pos, y_pos, *color);
    }

    pub fn place_pixel_rgb(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        self.place_pixel_rgba(x, y, &Rgba([color.0[0], color.0[1], color.0[2], 255]));
    }

    pub fn add_sphere(&mut self, x_position: f64, y_position: f64, z_position: f64, radius: f64) {
        let sphere = Sphere::new(x_position, y_position, z_position, radius, self.current_color);
        self.shapes.push(Box::new(sphere));
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.current_color = Rgba([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, 255]);
    }

    pub fn add_sun(&mut self, x_position: f64, y_position: f64, z_position: f64) {
        
    }
}
