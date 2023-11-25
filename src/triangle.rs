use image::DynamicImage;

use crate::{shapes::{RayShape, Point, ShapeType, MIN_ALLOWED_COLLISION_DISTANCE}, vector3::Vector3, bulb::Bulb, sun::Sun, ray::Ray};

#[derive(Debug)]
pub struct TriangleCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub s: f64,
    pub t: f64,
}

pub struct Triangle {
    first_point: Point,
    second_point: Point,
    third_point: Point,
    first_texture_vertex: (f64, f64),
    second_texture_vertex: (f64, f64),
    third_texture_vertex: (f64, f64),
    normal: Vector3,
    area: f64,
    color: [f64; 4],
    texture: Option<Vec<f32>>,
    first_edge_vec: Vector3,
    second_edge_vec: Vector3,
    third_edge_vec: Vector3,
}

impl RayShape for Triangle {
    fn get_center(&self) -> crate::shapes::Point {
        self.first_point
    }

    fn get_shape(&self) -> crate::shapes::ShapeType {
        ShapeType::Triangle
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
            let new_red = (self.color[0] * sun.get_color()[0] * lambert);
            let new_green = (self.color[1] * sun.get_color()[1] * lambert); 
            let new_blue = (self.color[2] * sun.get_color()[2] * lambert);
            return [new_red, new_green, new_blue, self.color[3]];
        }
        [0.0, 0.0, 0.0, 1.0]
    }

    fn get_color_without_sun(&self) -> [f64; 4] {
        self.color
    }

    fn get_intersection(&self, ray: &crate::ray::Ray) -> Option<(f64, crate::shapes::Point)> {
        let distance = self.first_point - ray.get_origin();
        let numerator = distance.to_vec().dot(self.normal);
        let denominator = ray.get_direction().dot(self.normal);
        let t = numerator / denominator;
        if t > 0.0 {
            let point = ray.get_origin() + t * ray.get_direction();
            let edge0 = self.second_point - self.first_point;
            let vec_point0 = point - self.first_point;
            let mut edge_cross_product = edge0.to_vec().cross(vec_point0.to_vec());
            let first_bary = (edge_cross_product.magnitude() / 2.0) / self.area;
            if self.normal.dot(edge_cross_product) < 0.0 {
                return None;
            }
            let edge1 = self.third_point - self.second_point;
            let vp1 = point - self.second_point;
            edge_cross_product = edge1.to_vec().cross(vp1.to_vec());
            let second_bary = (edge_cross_product.magnitude() / 2.0) / self.area;
            if self.normal.dot(edge_cross_product) < 0.0 {
                return None;
            }
            let edge2 = self.first_point - self.third_point;
            let vp2 = point - self.third_point;
            edge_cross_product = edge2.to_vec().cross(vp2.to_vec());
            let third_bary = (edge_cross_product.magnitude() / 2.0) / self.area;
            if self.normal.dot(edge_cross_product) < 0.0 {
                return None;
            }
            return Some((t, point));
        }
        None
    }

    fn get_normal_at_intersection(&self, intersection_point: &crate::shapes::Point) -> crate::vector3::Vector3 {
        if self.normal.z < 0.0 {
            return -self.normal;
        }
        self.normal
    }
}

impl Triangle {
    pub fn new(first_vertex: &TriangleCoordinate, second_vertex: &TriangleCoordinate, third_vertex: &TriangleCoordinate, color: [f64; 4], texture: Option<Vec<f32>>) -> Self {
        let p0 = Vector3::new(first_vertex.x, first_vertex.y, first_vertex.z);
        let p1 = Vector3::new(second_vertex.x, second_vertex.y, second_vertex.z);
        let p2 = Vector3::new(third_vertex.x, third_vertex.y, third_vertex.z);
        let p0_p1_vec = p1 - p0;
        let p2_p0_vec = p2 - p0;
        let p2_p1_vec = p2 - p1;
        let normal = p0_p1_vec.cross(p2_p0_vec).normalize();
        let first_edge_vec = p0_p1_vec.cross(normal).normalize();
        let second_edge_vec = p2_p0_vec.cross(normal).normalize();
        let third_edge_vec = p2_p1_vec.cross(normal).normalize();
        Self {
            first_point: Point::new(first_vertex.x, first_vertex.y, first_vertex.z),
            second_point: Point::new(second_vertex.x, second_vertex.y, second_vertex.z),
            third_point: Point::new(third_vertex.x, third_vertex.y, third_vertex.z),
            first_texture_vertex: (first_vertex.s, first_vertex.t),
            second_texture_vertex: (second_vertex.s, second_vertex.t),
            third_texture_vertex: (third_vertex.s, third_vertex.t),
            color,
            texture,
            normal,
            area: normal.magnitude() / 2.0,
            first_edge_vec,
            second_edge_vec,
            third_edge_vec,
        }
    }
}