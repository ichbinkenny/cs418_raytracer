use std::{fs::read_to_string, str::FromStr};

use crate::image_manip::RaytraceImage;

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
        if let Some(mut image) = self.image.as_mut() {
            image.add_sphere(x_position, y_position, z_position, radius);
        }
    }

    fn set_color(&mut self, r: f64, g: f64, b: f64) {
        if let Some(mut image) = self.image.as_mut() {
            image.set_color(r, g, b);
        }
    }

    fn add_sun(&mut self, x_position: f64, y_position: f64, z_position: f64) {
        if let Some(mut image) = self.image.as_mut() {
            image.add_sun(x_position, y_position, z_position);
        }
    }

    fn parse_command(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split(' ').collect();
        if tokens.len() > 0 {
            match tokens[0].to_ascii_lowercase().as_str() {
                "png" => self.create_png_image(u32::from_str_radix(tokens[1], 10).unwrap(), u32::from_str_radix(tokens[2], 10).unwrap(), tokens[3]),
                "sphere" => self.draw_sphere(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap(), f64::from_str(tokens[4]).unwrap()),
                "color" => self.set_color(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                "sun" => self.add_sun(f64::from_str(tokens[1]).unwrap(), f64::from_str(tokens[2]).unwrap(), f64::from_str(tokens[3]).unwrap()),
                _ => {},
            }
        }
    }
}
