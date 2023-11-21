use crate::{shapes::{RayShape, Point, ShapeType, MIN_ALLOWED_COLLISION_DISTANCE}, vector3::Vector3, bulb::Bulb, sun::Sun, ray::Ray};

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
    color: [f64; 4],
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
        self.color
    }

    fn get_color_without_sun(&self) -> [f64; 4] {
        self.color
    }

    fn get_intersection(&self, ray: &crate::ray::Ray) -> Option<(f64, crate::shapes::Point)> {
        // lookup ray point barycentric coordinates.
        // first, find distance to hit. This is basically a fancy plane.
        let distance = self.first_point - ray.get_origin();
        let numerator = distance.to_vec().dot(self.normal);
        let denominator = ray.get_direction().dot(self.normal);
        let t = numerator / denominator;
        if t > 0.0 {
            let collision_point = (t * ray.get_direction() + ray.get_origin().to_vec()).to_point();
            // We have a bit more work to do here. We need to see if the point lies on our triangle.
            // Prescale?
            // println!("Checking point {:?}", collision_point);
            let first_bary = self.first_edge_vec.dot(collision_point.to_vec());
            let second_bary = self.second_edge_vec.dot(collision_point.to_vec());
            let third_bary = self.third_edge_vec.dot(collision_point.to_vec());
            let bary_sum = first_bary + second_bary + third_bary;
            // If any of our barycentric coordinates are negative, we are not in the triangle.
            if first_bary.is_sign_negative() || second_bary.is_sign_negative() || third_bary.is_sign_negative() {
                // println!("Invalid");
                return None;
            }
            println!("First bary {:?} Second bary {:?} Third bary {:?}", first_bary, second_bary, third_bary);
            let p_prime = (t * ray.get_direction() + ray.get_origin().to_vec()).to_point();
            return Some((t, p_prime));
        }
        None
    }

    fn get_normal_at_intersection(&self, intersection_point: &crate::shapes::Point) -> crate::vector3::Vector3 {
        self.normal
    }
}

impl Triangle {
    pub fn new(first_vertex: &TriangleCoordinate, second_vertex: &TriangleCoordinate, third_vertex: &TriangleCoordinate, color: [f64; 4]) -> Self {
        let p0 = Vector3::new(first_vertex.x, first_vertex.y, first_vertex.z);
        let p1 = Vector3::new(second_vertex.x, second_vertex.y, second_vertex.z);
        let p2 = Vector3::new(third_vertex.x, third_vertex.y, third_vertex.z);
        let p0_p1_vec = p1 - p0;
        let p2_p0_vec = p2 - p0;
        let p2_p1_vec = p2 - p1;
        let normal = p0_p1_vec.cross(p2_p0_vec).normalize();
        let first_edge_vec = p0_p1_vec.cross(normal);
        let second_edge_vec = p2_p0_vec.cross(normal);
        let third_edge_vec = p2_p1_vec.cross(normal);
        Self {
            first_point: Point::new(first_vertex.x, first_vertex.y, first_vertex.z),
            second_point: Point::new(second_vertex.x, second_vertex.y, second_vertex.z),
            third_point: Point::new(third_vertex.x, third_vertex.y, third_vertex.z),
            first_texture_vertex: (first_vertex.s, first_vertex.t),
            second_texture_vertex: (second_vertex.s, second_vertex.t),
            third_texture_vertex: (third_vertex.s, third_vertex.t),
            color,
            normal,
            first_edge_vec,
            second_edge_vec,
            third_edge_vec,
        }
    }
}