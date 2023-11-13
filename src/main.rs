mod image_manip;
mod input_parser;
mod shapes;
mod sphere;
mod ray;
mod vector3;
mod sun;

use std::env::args;
use input_parser::RaytraceInputParser;
fn main() {
    let base_directory = String::from(env!("CARGO_MANIFEST_DIR")) + "/";
    let file_path = base_directory + &args().nth(1).unwrap();
    let mut command_parser = RaytraceInputParser::new(&file_path);
    command_parser.create_image();
    command_parser.render();
}

