use std::{fs::read_to_string, str::FromStr};

use crate::{image_manip::RaytraceImage, shapes::Point, vector3::Vector3, ray::RayLensType};

enum RaytraceCommand {
    Png(u32, u32),
    Sphere(i32, i32, i32, u32),
    Color3(f64, f64, f64),
    Sun(i32, i32, i32), // Contains the (x, y, z) direction for light source
}

pub struct RaytraceInputParser {
    lines: Vec<String>,
    image: Option<RaytraceImage>,
}

impl RaytraceInputParser {
    pub fn new(src_file: &str) -> Self {
        let file_contents = match read_to_string(src_file) {
            Ok(file_data) => file_data,
            Err(_) => panic!("Unable to find file {}", src_file),
        };
        let mut data_lines = vec![];
        for line in file_contents.lines() {
            println!("{}", line);
            data_lines.push(line.to_string());
        }
        Self {
            lines:  data_lines,
            image: None,
        }
    }

    pub fn render(&mut self) {
        if let Some(ref mut img) = self.image {
            img.render();
            img.save();
        }
    }

    pub fn create_image(&mut self) {
        for line in &self.lines.clone() {
            self.parse_command(&line);
        }
        
        // Make sure we actually have a PNG image ready and save it
        if let Some(image) = &self.image {
            image.save();
        }
    }

    fn create_png_image(&mut self, width: u32, height: u32, output_file: &str) {
        self.image = Some(RaytraceImage::new(width, height, output_file));
    }

    fn draw_sphere(&mut self, x_position: f64, y_position: f64, z_position: f64, radius: f64) {
        if let Some(image) = self.image.as_mut() {
            image.add_sphere(x_position, y_position, z_position, radius);
        }
    }

    fn set_color(&mut self, r: f64, g: f64, b: f64) {
        if let Some(image) = self.image.as_mut() {
            image.set_color(r, g, b);
        }
    }

    fn add_sun(&mut self, x_position: f64, y_position: f64, z_position: f64) {
        if let Some(image) = self.image.as_mut() {
            image.add_sun(x_position, y_position, z_position);
        }
    }

    fn set_exposure(&mut self, exposure_value: f64) {
        if let Some(image) = self.image.as_mut() {
            image.set_exposure(exposure_value);
        }
    }

    fn add_plane(&mut self, normal_x: f64, normal_y: f64, normal_z: f64, d_val: f64) {
        if let Some(image) = self.image.as_mut() {
            image.add_plane(normal_x, normal_y, normal_z, d_val);
        }
    }

    fn set_eye_origin(&mut self, origin: &Point) {
        if let Some(image) = self.image.as_mut() {
            image.set_eye_origin(origin);
        }
    }

    fn set_forward_direction(&mut self, direction: &Vector3) {
        if let Some(image) = self.image.as_mut() {
            image.set_forward_direction(direction);
        }
    }

    fn set_up_direction(&mut self, direction: &Vector3) {
        if let Some(image) = self.image.as_mut() {
            image.set_up_direction(direction);
        }
    }

    fn use_fisheye_lens(&mut self) {
        if let Some(image) = self.image.as_mut() {
            image.set_lens(RayLensType::Fisheye);
        }
    }

    fn use_panoramic_lens(&mut self) {
        if let Some(image) = self.image.as_mut() {
            image.set_lens(RayLensType::Panorama);
        }
    }

    fn add_triangle_vertex(&mut self, x: f64, y: f64, z: f64) {
        if let Some(image) = self.image.as_mut() {
            image.add_triangle_vertex(x, y, z);
        }
    }

    fn add_triangle(&mut self, t1: i32, t2: i32, t3: i32) {
        if let Some(image) = self.image.as_mut() {
            image.add_triangle(t1 - 1, t2 - 1, t3 - 1);
        }
    }

    fn add_bulb(&mut self, x: f64, y: f64, z: f64) {
        if let Some(image) = self.image.as_mut() {
            image.add_bulb(x, y, z);
        }
    }

    fn set_shininess(&mut self, values: Vec<String>) {
        if let Some(image) = self.image.as_mut() {
            if values.len() == 1 {
                let shininess = f64::from_str(values[0].as_str()).unwrap();
                image.set_shininess([shininess, shininess, shininess]);
            }
            else {
                image.set_shininess([f64::from_str(values[0].as_str()).unwrap(), f64::from_str(values[1].as_str()).unwrap(), f64::from_str(values[2].as_str()).unwrap()]);
            }
        }
    }

    fn set_antialias_value(&mut self, num_rays: u32) {
        if let Some(image) = self.image.as_mut() {
            image.set_antialias_value(num_rays);
        }
    }

    fn parse_command(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split(' ').filter(|e| { *e != "" }).collect();
        if tokens.len() > 0 {
            match tokens[0].to_ascii_lowercase().as_str() {
                "png" => self.create_png_image(u32::from_str_radix(tokens[1], 10).unwrap(), u32::from_str_radix(tokens[2], 10).unwrap(), tokens[3]),
                "sphere" => self.draw_sphere(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap(), f64::from_str(tokens[4]).unwrap()),
                "color" => self.set_color(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                "sun" => self.add_sun(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                "expose" => self.set_exposure(f64::from_str(tokens[1]).unwrap()),
                "plane" => self.add_plane(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap(), f64::from_str(tokens[4]).unwrap()),
                "eye" => self.set_eye_origin(&Point::new(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap())),
                "forward" => self.set_forward_direction(&Vector3::new(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap())),
                "up" => self.set_up_direction(&Vector3::new(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap())),
                "fisheye" => self.use_fisheye_lens(),
                "panorama" => self.use_panoramic_lens(),
                "xyz" => self.add_triangle_vertex(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                "tri" => self.add_triangle(i32::from_str(tokens[1]).unwrap(), i32::from_str(tokens[2]).unwrap(), i32::from_str(tokens[3]).unwrap()),
                "bulb" => self.add_bulb(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                "shininess" => self.set_shininess(tokens[1..].iter().map(|e| String::from(*e)).collect()),
                "aa" => self.set_antialias_value(u32::from_str(tokens[1]).unwrap()),
                _ => {},
            }
        }
    }
}
