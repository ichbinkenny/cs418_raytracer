use std::collections::HashMap;
use std::f64::consts::E;
use std::fs::File;
use std::io::BufReader;

use image::codecs::png::PngReader;
use image::{Rgba, RgbaImage, Rgb, DynamicImage};
use crate::bulb::Bulb;
use crate::shapes::{RayShape, Point};
use crate::ray::{Ray, RayLensType};
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::triangle::{TriangleCoordinate, Triangle};
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
    exposure_value: Option<f64>,
    lens_type: RayLensType,
    current_texture: Option<Vec<f32>>,
    texture_coord: (f64, f64),
    triangle_verticies: Vec<TriangleCoordinate>,
    bulbs: Vec<Bulb>,
    shininess: [f64; 3],
    bounces: u8,
    num_antialias_rays: u32,
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
            exposure_value: None,
            lens_type: RayLensType::Standard,
            current_texture: None,
            texture_coord: (0.0, 0.0),
            triangle_verticies: vec![],
            bulbs: vec![],
            shininess: [0.0, 0.0, 0.0],
            bounces: 4,
            num_antialias_rays: 0,
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
                    let ray = Ray::new(self.forward_direction, self.up_direction, self.right_direction, self.ray_origin, s_x, s_y, self.lens_type);
                    // For each shape at this pixel,
                    // Get the distance to intersection
                    // Choose the smallest intersection to be the color
                    let shape = &self.shapes[i];
                    match shape.get_intersection(&ray) {
                        Some(data) => {
                            if !found_shape {
                                if data.0.is_nan() {
                                    //continue;
                                }
                                smallest_distance = data.0;
                                found_shape = true;
                                let mut color = [0.0, 0.0, 0.0, 0.0];
                                self.pixels.insert((x as usize, y as usize), Rgba([0, 0, 0, 0]));
                                for sun in &self.suns {
                                    let next_color = shape.get_linear_color(&self.shapes, &self.bulbs, &ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], next_color[3] + color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0]  as f64), (color[1]   as f64), (color[2] as f64), color[3] / self.suns.len() as f64];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_srgb(color, self.exposure_value));
                                
                            } else if data.0 < smallest_distance {
                                if data.0.is_nan() {
                                    //continue;
                                }
                                smallest_distance = data.0;
                                let mut color = [0.0, 0.0, 0.0, 0.0];
                                for sun in &self.suns {
                                    let next_color = shape.get_linear_color(&self.shapes, &self.bulbs, &ray, &sun, &data.1);
                                    color = [color[0] + next_color[0], color[1] + next_color[1], color[2] + next_color[2], next_color[3] + color[3]];
                                }
                                if self.suns.len() > 0 {
                                    color = [(color[0] / self.suns.len() as f64), (color[1]  / self.suns.len() as f64), (color[2] / self.suns.len() as f64), color[3] / self.suns.len() as f64];
                                }
                                self.pixels.insert((x as usize, y as usize), RaytraceImage::linear_color_to_srgb(color, self.exposure_value));
                            }
                        },
                        None => {
                        },
                    }
                    
                }

                // Place closest shape color
                if found_shape {
                    let color = self.pixels[&(x as usize, y as usize)];
                    if x == 0 && y == 0 {
                        println!("using color: {:?}", color);
                    }
                    self.place_pixel_rgba(x, y, &color);
                }
                else {
                    self.place_pixel_rgba(x, y, &Rgba([0, 0, 0, 0]));
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
        let converted_color = Rgba([color[0].min(255), color[1].min(255), color[2].min(255), color[3]]);
        self.image.put_pixel(x_pos, y_pos, converted_color);
    }

    pub fn place_pixel_rgb(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        self.place_pixel_rgba(x, y, &Rgba([color.0[0], color.0[1], color.0[2], 255]));
    }

    pub fn add_sphere(&mut self, x_position: f64, y_position: f64, z_position: f64, radius: f64) {
        let sphere = Sphere::new(x_position, y_position, z_position, radius, self.current_color, self.current_texture.clone(), self.shininess);
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
        let plane = Plane::new(x, y, z, d, self.current_color, self.shininess);
        println!("Creating plane with color {:?}", self.current_color);
        self.shapes.push(Box::new(plane));
    }

    pub fn set_exposure(&mut self, exposure_value: f64) {
        self.exposure_value = Some(exposure_value);
    }

    pub fn set_lens(&mut self, lens_type: RayLensType) {
        self.lens_type = lens_type;
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
        self.up_direction = self.right_direction.cross(self.forward_direction).normalize();
    }

    pub fn add_triangle_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.triangle_verticies.push(TriangleCoordinate { x: x, y: y, z: z, s: self.texture_coord.0, t: self.texture_coord.1 });
    }

    pub fn add_triangle(&mut self, i1: i32, i2: i32, i3: i32) {
        // We are using a 1 based index, with negative numbers meaning from the back of the triangle verticies vector.
        let mut index1 = i1;
        let mut index2 = i2;
        let mut index3 = i3;
        if i1 < 0 {
            index1 = self.triangle_verticies.len() as i32 - i1.abs() + 1;
        }
        if i2 < 0 {
            index2 = self.triangle_verticies.len() as i32 - i2.abs() + 1;
        }
        if i3 < 0 {
            index3 = self.triangle_verticies.len() as i32 - i3.abs() + 1;
        }
        let first_point = &self.triangle_verticies[index1 as usize];
        let second_point = &self.triangle_verticies[index2 as usize];
        let third_point = &self.triangle_verticies[index3 as usize];
        self.shapes.push(Box::new(Triangle::new(first_point, second_point, third_point, self.current_color, self.current_texture.clone())));
    }

    pub fn set_texture(&mut self, texture_file: &str) {
        let image_file = match std::fs::File::open(texture_file) {
            Ok(res) => BufReader::new(res),
            Err(_) => return,
        };
        match image::load(image_file, image::ImageFormat::Png) {
            Ok(tex) => {
                println!("Loaded texture: {}", texture_file);
                self.current_texture = Some(tex.into_rgba32f().to_vec());
            },
            Err(_) => self.current_texture = None,
        };
    }

    pub fn add_bulb(&mut self, x: f64, y: f64, z: f64) {
        let bulb = Bulb::new(Point::new(x, y, z), self.current_color);
        self.bulbs.push(bulb);
    }

    pub fn set_shininess(&mut self, shininess: [f64; 3]) {
        self.shininess = shininess;
    }

    pub fn set_antialias_value(&mut self, num_rays: u32) {
        self.num_antialias_rays = num_rays;
    }

    pub fn color_from_exposure(color: [f64; 4], exposure_value: f64) -> [f64; 4] {
        let exposure_red = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[0]);
        let exposure_green = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[1]);
        let exposure_blue = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[2]);
        [exposure_red, exposure_green, exposure_blue, color[3]]
    }

    fn linear_color_to_srgb(color: [f64; 4], exposure_value: Option<f64>) -> Rgba<u8> {
        // Check to see if we need to apply exposure
        let mut linear_color = color;
        if let Some(exposure) = exposure_value {
            linear_color = RaytraceImage::color_from_exposure(linear_color, exposure);
        }
        let mut output_color = [0.0, 0.0, 0.0, linear_color[3]]; 
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
        let output_color_u8 = [(output_color[0] * 255.0).floor().min(255.0) as u8, (output_color[1] * 255.0).floor().min(255.0) as u8, (output_color[2] * 255.0).floor().min(255.0) as u8, 255.0 as u8];
        return Rgba(output_color_u8);
        // println!("Converted: {:?} to sRGB: {:x?}", linear_color, output_color_u8);
        
    } 

}
