use std::f64::consts::E;

use image::Rgba;

pub struct RgbaUtil;

pub const LINEAR_SRGB_CUTOFF: f64 = 0.0031308;
pub const SRGB_LINEAR_CUTOFF: f64 = 0.04045;

impl RgbaUtil {
    pub fn rgba_to_linear(color: [f64; 4]) -> [f64; 4] {
        let mut output_color = [0.0, 0.0, 0.0, 0.0];
        if color[0] <= SRGB_LINEAR_CUTOFF {
            output_color[0] = color[0] / 12.92;
        } else {
            output_color[0] = ((color[0] + 0.055) / 1.055).powf(2.4);
        }
        if color[1] <= SRGB_LINEAR_CUTOFF {
            output_color[1] = color[1] / 12.92;
        } else {
            output_color[1] = ((color[1] + 0.055) / 1.055).powf(2.4);
        }
        if color[2] <= SRGB_LINEAR_CUTOFF {
            output_color[2] = color[2] / 12.92;
        } else {
            output_color[2] = ((color[2] + 0.055) / 1.055).powf(2.4);
        }
        output_color
    }

    pub fn linear_to_rgba(color: [f64; 4], exposure_value: Option<f64>) -> Rgba<u8> {
        let mut linear_color = color;
        if let Some(exposure) = exposure_value {
            linear_color = RgbaUtil::color_from_exposure(linear_color, exposure);
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
    }

    pub fn color_from_exposure(color: [f64; 4], exposure_value: f64) -> [f64; 4] {
        let exposure_red = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[0]);
        let exposure_green = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[1]);
        let exposure_blue = 1.0f64 - E.powf(-1.0f64 * exposure_value * color[2]);
        [exposure_red, exposure_green, exposure_blue, color[3]]
    }
}

#[cfg(test)]
mod rgba_util_tests {
    use super::RgbaUtil;

    #[test]
    fn rgba_to_linear_midpoint_within_one_percent() {
        let color = [0.735, 0.735, 0.735, 1.0];
        let c_max = 0.5 + (0.02 * 0.5);
        let c_min = 0.5 - (0.02 * 0.5);
        let result = RgbaUtil::rgba_to_linear(color);
        println!("MAX: {}, MIN: {}", c_max, c_min);
        println!("Result: {:?}", result);
        let mut less_than_one_percent = true;
        result.map(|c| { less_than_one_percent = c < c_max && c > c_min});
        assert!(less_than_one_percent);
    }
}