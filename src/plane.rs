use std::thread::current;

use crate::{shapes::{ShapeType, Point, RayShape, MIN_ALLOWED_COLLISION_DISTANCE}, vector3::Vector3, ray::Ray, sun::Sun, bulb::Bulb};

pub struct Plane {
    normal: Vector3,
    point: Point,
    color: [f64; 4],
    shininess: [f64; 3],
}

impl RayShape for Plane {
    fn get_center(&self) -> Point {
        self.point
    }
    fn get_shape(&self) -> ShapeType {
        ShapeType::Plane
    }
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Point)> {
        let distance = self.point - ray.get_origin();
        let numerator = distance.to_vec().dot(self.normal);
        let denominator = ray.get_direction().dot(self.normal);
        let t = numerator / denominator;
        if t > 0.0 {
            let p_prime = (t * ray.get_direction() + ray.get_origin().to_vec()).to_point();
            return Some((t, p_prime));
        }
        None
    }

    fn get_linear_color(&self, objects: &Vec<Box<dyn RayShape>>, bulbs: &Vec<Bulb>, ray: &Ray, sun: &Sun, intersection_point: &Point) -> [f64; 4] {
        let light_direction = sun.light_direction.normalize();
        let collision_info = self.get_intersection(ray).unwrap();
        let current_distance = collision_info.0;
        let collision_point = collision_info.1;
        let normal = self.get_normal_at_intersection(&collision_point);
        let lambert = normal.dot(light_direction).max(0.0);
        let shadow_ray = Ray {origin: collision_point, direction: light_direction, lens_type: ray.lens_type, generation: ray.generation + 1, max_bounces: ray.max_bounces};
        let mut in_shadow = false;
        for obj in objects {
            if let Some(hit_point) = obj.get_intersection(&shadow_ray) {
                if hit_point.0 < current_distance && hit_point.0 > MIN_ALLOWED_COLLISION_DISTANCE {
                    in_shadow = true;
                    break;
                }
            }
        }
        if !in_shadow {
            let new_red = self.color[0] * sun.get_color()[0] * lambert;
            let new_green = self.color[1] * sun.get_color()[1] * lambert; 
            let new_blue = self.color[2] * sun.get_color()[2] * lambert;
            return [new_red, new_green, new_blue, self.color[3]];
        }
        [0.0, 0.0, 0.0, 1.0]
    }

    fn get_color_without_sun(&self) -> [f64; 4] {
        todo!()
    }

    fn get_normal_at_intersection(&self, _intersection_point: &Point) -> Vector3 {
        self.normal
    }
}

impl Plane {
    pub fn new(x: f64, y: f64, z: f64, d: f64, color: [f64; 4], shininess: [f64; 3]) -> Self {
        let point_from_eq = ((-d * Vector3::new(x, y, z)) * ( 1.0 / (x.powi(2) + y.powi(2) + z.powi(2)).sqrt())).to_point();
        Self{
            normal: Vector3::new(x, y, z).normalize(),
            point: point_from_eq,
            color,
            shininess
        }
    } 
}
