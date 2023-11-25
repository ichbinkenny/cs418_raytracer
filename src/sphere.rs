use image::DynamicImage;

use crate::{shapes::{Point, ShapeType, RayShape, self, MIN_ALLOWED_COLLISION_DISTANCE}, ray::Ray, sun::Sun, vector3::Vector3, bulb::Bulb};

pub struct Sphere {
    center: Point, 
    radius: f64,
    color: [f64; 4],
    texture: Option<Vec<f32>>,
    shininess: [f64; 3],
}

impl RayShape for Sphere {
    fn get_center(&self) -> Point {
        self.center
    }
    fn get_shape(&self) -> ShapeType {
        ShapeType::Sphere
    }
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Point)> {
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
            return Some((t, c_double_prime.to_point()));
        }
        // Intersection out of sphere
        let t = to_center_hitpoint - hit_offset;
        let c_double_prime = t * ray.get_direction() + ray.get_origin().to_vec();
        Some((t, c_double_prime.to_point()))
    }

    ///
    /// Returns the current pixel color in linear RGB. Needs to be converted to sRGB.
    /// 
    fn get_linear_color(&self, objects: &Vec<Box<dyn RayShape>>, bulbs: &Vec<Bulb>, ray: &Ray, sun: &Sun, intersection_point: &Point) -> [f64; 4] {
        // Find the point we have hit on the sphere. This is simply the origin of the ray + its direction times by the distance we returned.
        let collision_point = ray.get_origin() + (ray.get_direction() * intersection_point.to_vec().magnitude());
        let intersection_info = self.get_intersection(ray).unwrap();
        // Get the surface normal, like usual.
        let normal = self.get_normal_at_intersection(&collision_point);
        // Find l_d (light direction) from this collision point
        let light_direction = sun.light_direction.normalize();
        // get lambert color
        let lambert = normal.dot(light_direction).max(0.0);
        // Create a new ray pointed back from our surface
        let secondary_shadow_ray = Ray { origin: collision_point, direction: light_direction, lens_type: ray.lens_type, generation: ray.generation + 1, max_bounces: ray.max_bounces};
        let mut in_shadow = false;

        for obj in objects {
            if let Some(intersection) = obj.get_intersection(&secondary_shadow_ray) {
                if intersection.0 < intersection_info.0 && intersection.0 > MIN_ALLOWED_COLLISION_DISTANCE {

                    in_shadow = true;
                    break;
                }
            }
        }

        //TODO: Bulb lighting?
        let mut bulb_color = [0.0, 0.0, 0.0, 1.0];
        for bulb in bulbs {
            bulb_color[0] += bulb.get_light(collision_point)[0];
            bulb_color[1] += bulb.get_light(collision_point)[1];
            bulb_color[2] += bulb.get_light(collision_point)[2];
        }
        bulb_color = bulb_color.map(|b| b / bulbs.len() as f64);
        // return color
        match &self.texture {
            Some(tex) => {
                if !in_shadow && bulbs.len() == 0 {
                    //HELP
                    let pixel_index = ((4.0 * intersection_point.x)) as usize;
                    let new_red = tex[pixel_index] as f64;
                    let new_green = tex[pixel_index + 1] as f64;
                    let new_blue = tex[pixel_index + 2] as f64;
                    let new_alpha = tex[pixel_index + 3] as f64;
                    return [new_red, new_green, new_blue, new_alpha];
                }
            },
            None => {
                if !in_shadow && bulbs.len() == 0 {
                    // Get our ratio of reflected light to diffused.
                    let mut new_red = self.color[0] * sun.get_color()[0] * (lambert * (1.0 - self.shininess[0]));
                    let mut new_green = self.color[1] * sun.get_color()[1] * (lambert * (1.0 - self.shininess[1])); 
                    let mut new_blue = self.color[2] * sun.get_color()[2] * (lambert * (1.0 - self.shininess[2]));
                    let new_ray_direction = light_direction - (2.0 * normal.dot(light_direction)) * normal;
                    let reflection_ray = Ray { origin: collision_point, direction: new_ray_direction, lens_type: ray.lens_type, generation: ray.generation + 1, max_bounces: ray.max_bounces};
                    if reflection_ray.generation < reflection_ray.max_bounces {
                        //TODO
                    }
                    return [new_red, new_green, new_blue, self.color[3]];
                }
                else if !in_shadow {
                    // Get our ratio of reflected light to diffused.
                    let new_red = self.color[0] * sun.get_color()[0] * lambert + lambert * bulb_color[0];
                    let new_green = self.color[1] * sun.get_color()[1] * lambert + lambert * bulb_color[1]; 
                    let new_blue = self.color[2] * sun.get_color()[2] * lambert + lambert * bulb_color[2];
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
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, color: [f64; 4], texture: Option<Vec<f32>>, shininess: [f64; 3]) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            color,
            texture,
            shininess
        }
    }
}