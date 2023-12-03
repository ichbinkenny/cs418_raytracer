
use std::{f64::consts::PI, any::Any};

use image::Rgba32FImage;

use crate::{shapes::{Point, ShapeType, RayShape, MIN_ALLOWED_COLLISION_DISTANCE}, ray::Ray, sun::Sun, vector3::Vector3, bulb::{Bulb, self}, rgba_utils::RgbaUtil};

pub struct Sphere {
    center: Point, 
    radius: f64,
    color: [f64; 4],
    texture: Option<Rgba32FImage>,
    texture_width: Option<f64>,
    texture_height: Option<f64>,
    shininess: [f64; 3],
    transparency: [f64; 3],
    index_of_refraction: f64,
}

impl RayShape for Sphere {
    fn get_center(&self) -> Point {
        self.center
    }
    fn get_shape(&self) -> ShapeType {
        ShapeType::Sphere
    }

    fn equals(&self, rhs: &dyn Any) -> bool {
        if let Some(other) = rhs.downcast_ref::<Sphere>() {
            return other.get_center() == self.get_center() && other.radius == self.radius && other.color == self.color;
        }
        false
    }

    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Point, [f64;3])> {
        let center_sub_origin = (self.center - ray.get_origin()).to_vec();
        // Compare our vector length to see if it is shorter than the size of the sphere's radius.
        // If so, we are entirely inside the sphere.
        let inside = center_sub_origin.magnitude().powi(2) < self.radius.powi(2);
        let to_center_hitpoint = center_sub_origin.dot(ray.get_direction()) / ray.get_direction().magnitude();
        // If we're not inside the sphere, and register a hit distance that is negative, the sphere is behind the camera.
        // No intersection
        if !inside && to_center_hitpoint < 0.0 {
            return None;
        }
        let distance_squared = (ray.get_origin() + to_center_hitpoint * ray.get_direction() - self.center).to_vec().magnitude().powi(2);
        // distance squared can only be > than the radius squared if a ray was outside the sphere. Otherwise no intersection.
        if distance_squared > self.radius.powi(2) && !inside {
            return None;
        }
        // Get the offset of the hit point from the camera and our central point.
        // Each intersection for a sphere forms a cord, which requires us to find the closer of the two intersections.
        // find the offset between t and t_c. Then t +- t_c will give the two intersection points.
        let hit_offset = (self.radius.powi(2) - distance_squared).sqrt() / ray.get_direction().magnitude();
        // If we're inside, return t_c + hit offset
        if inside {
            let t = to_center_hitpoint + hit_offset;
            let c_double_prime = t * ray.get_direction() + ray.get_origin().to_vec();
            return Some((t, c_double_prime.to_point(), [0.0, 0.0, 0.0]));
        }
        // Intersection out of sphere
        let t = to_center_hitpoint - hit_offset;
        let c_double_prime = t * ray.get_direction() + ray.get_origin().to_vec();
        Some((t, c_double_prime.to_point(), [0.0, 0.0, 0.0]))
    }

    ///
    /// Returns the current pixel color in linear RGB. Needs to be converted to sRGB.
    /// 
    fn get_linear_color(&self, objects: &Vec<Box<dyn RayShape>>, bulbs: &Vec<Bulb>, ray: &Ray, sun: &Sun, intersection_point: &Point) -> [f64; 4] {
        if ray.generation > ray.max_bounces {
            println!("Hit max bounce of : {}", ray.generation);
            return [0.0, 0.0, 0.0, 1.0];
        }
        let intersection_info = self.get_intersection(ray).unwrap();
        // Get the surface normal, like usual.
        let normal = self.get_normal_at_intersection(&intersection_info.1).normalize();
        // Find l_d (light direction) from this collision point
        let light_direction = sun.light_direction.normalize();
        // get lambert color
        let lambert = normal.dot(light_direction).max(0.0);
        // Create a new ray pointed back from our surface
        let secondary_shadow_ray = Ray { origin: intersection_info.1, direction: light_direction.normalize(), lens_type: ray.lens_type, generation: ray.generation + 1, max_bounces: ray.max_bounces};
        let mut in_shadow = false;

        for obj in objects {
            if let Some(intersection) = obj.get_intersection(&secondary_shadow_ray) {
                if intersection.0 < intersection_info.0 && !obj.equals(self){

                    in_shadow = true;
                    break;
                }
            }
        }

        //TODO: Bulb lighting?
        let mut bulb_color = [0.0, 0.0, 0.0, 1.0];
        for bulb in bulbs {
            bulb_color[0] += bulb.get_light(&intersection_info.1)[0];
            bulb_color[1] += bulb.get_light(&intersection_info.1)[1];
            bulb_color[2] += bulb.get_light(&intersection_info.1)[2];
        }
        bulb_color = bulb_color.map(|b| b / bulbs.len() as f64);
        // return color
        match &self.texture {
            Some(tex) => {
                if !in_shadow && bulbs.len() == 0 {
                    let n = (1.0 / self.radius) * (intersection_info.1 - self.center).to_vec();
                    let longitude = n.x.atan2(n.z);
                    let latitude = n.y.atan2((n.x.powi(2) + n.z.powi(2)).sqrt());
                    // Longitude from -PI to PI into 0 to 1
                    // Latitude from -PI/2 to PI/2
                    let corrected_longitude = (longitude + (PI / 2.0)) / (2.0 * PI);
                    let corrected_latitude = (-latitude + (PI / 2.0)) / PI;
                    let pixel_x = corrected_longitude * tex.width() as f64;
                    let pixel_y = corrected_latitude * tex.height() as f64;
                    let pixel = tex.get_pixel(pixel_x as u32, pixel_y as u32);
                    // Pixel color is RGBA :(
                    // Convert it to srgb
                    let converted_color = RgbaUtil::rgba_to_linear(pixel.0.map(|e| e as f64));
                    return [converted_color[0] * lambert * sun.get_color()[0], converted_color[1] * lambert * sun.get_color()[1], converted_color[2] * lambert * sun.get_color()[2], converted_color[3]];
                }
            },
            None => {
                if !in_shadow && bulbs.len() == 0 && self.shininess == [0.0, 0.0, 0.0] {
                    // Get our ratio of reflected light to diffused.
                    let mut new_red = self.color[0] * sun.get_color()[0] * lambert;
                    let mut new_green = self.color[1] * sun.get_color()[1] * lambert; 
                    let mut new_blue = self.color[2] * sun.get_color()[2] * lambert;
                    return [new_red, new_green, new_blue, self.color[3]];
                }
                else if !in_shadow && bulbs.len() == 0 {
                    // Reflection
                    let mut new_red = self.color[0] * sun.get_color()[0] * lambert;
                    let mut new_green = self.color[1] * sun.get_color()[1] * lambert; 
                    let mut new_blue = self.color[2] * sun.get_color()[2] * lambert;
                    let reflection_direction = ray.direction - (2.0 * (normal.dot(ray.direction)) * normal);
                    let reflection_ray = Ray {origin: intersection_info.1, direction: reflection_direction, lens_type: ray.lens_type, generation: ray.generation + 1, max_bounces: ray.max_bounces};
                    let diffuse_factor = [1.0 - self.shininess[0], 1.0 - self.shininess[1], 1.0 - self.shininess[2], 0.0];
                    let mut color = [new_red * diffuse_factor[0], new_green * diffuse_factor[1], new_blue * diffuse_factor[2], self.color[3]];
                    // Find next shape for color
                    let mut min_t = f64::MAX;
                    let mut reflection_point = intersection_info.1;
                    let mut hit_shape = None;
                    for obj in objects {
                        if let Some(hit_info) = obj.get_intersection(&reflection_ray) {
                            if min_t > hit_info.0  && !obj.equals(self) {
                                min_t = hit_info.0;
                                reflection_point = hit_info.1;
                                hit_shape = Some(obj);
                            }
                        }
                    }
                    let mut reflection_color = [0.0, 0.0, 0.0, 1.0];
                    if let Some(shape) = hit_shape {
                        let computed_color = shape.get_linear_color(objects, bulbs, &reflection_ray, sun, &reflection_point);
                        reflection_color = [computed_color[0] * self.shininess[0], computed_color[1] * self.shininess[1], computed_color[2] * self.shininess[2], 1.0];
                    }
                    color = [color[0] + reflection_color[0], color[1] + reflection_color[1], color[2] + reflection_color[2], color[3]];
                    return color;
                }
                else if bulbs.len() > 0 {
                    // Get our ratio of reflected light to diffused.

                        let new_red = (self.color[0] * (sun.get_color()[0] + bulb_color[0]) * lambert) ;
                        let new_green = (self.color[1] * (sun.get_color()[1] + bulb_color[1]) * lambert); 
                        let new_blue = (self.color[2] * (sun.get_color()[2] + bulb_color[2]) * lambert);
                        return [new_red, new_green, new_blue, self.color[3]];
                }
            }
        }
        return [0.0, 0.0, 0.0, 1.0];
    }

    fn get_color_without_sun(&self) -> [f64; 4] {
        self.color
    }

    fn get_normal_at_intersection(&self, intersection_point: &Point) -> crate::vector3::Vector3 {
        (*intersection_point - self.center).to_vec().normalize()
    }

    fn get_shininess(&self) -> [f64; 3] {
        self.shininess
    }
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, color: [f64; 4], texture: &Option<Rgba32FImage>, shininess: [f64; 3], transparency: [f64;3], index_of_refraction: f64, texture_width: Option<f64>, texture_height: Option<f64>) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            color,
            texture: texture.clone(),
            texture_width,
            texture_height,
            shininess,
            transparency,
            index_of_refraction,
        }
    }
}


#[cfg(test)]
mod sphere_tests {
    use crate::{shapes::RayShape, plane::Plane};

    use super::Sphere;

    #[test]
    fn sphere_equals_self() {
        let sphere = Sphere::new(0.0, 0.0, 0.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        assert!(sphere.equals(&sphere));
    }

    #[test]
    fn two_equal_spheres() {
        let sphere1 = Sphere::new(0.0, 0.0, 0.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        let sphere2 = Sphere::new(0.0, 0.0, 0.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        assert!(sphere1.equals(&sphere2));
    }

    #[test]
    fn two_unequal_spheres() {
        let sphere1 = Sphere::new(0.0, 0.0, 0.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        let sphere2 = Sphere::new(1.0, 0.0, 3.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        assert!(!sphere1.equals(&sphere2));
    }

    #[test]
    fn sphere_does_not_equal_plane() {
        let sphere1 = Sphere::new(0.0, 0.0, 0.0, 1.0, [0.0, 0.0, 0.0, 0.0], &None, [0.0, 0.0, 0.0], None, None);
        let plane = Plane::new(0.0, 0.0, 0.0, 0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        assert!(!sphere1.equals(&plane));
    }

}