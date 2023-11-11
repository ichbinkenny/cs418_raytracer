use std::ops::{Add, Sub};

use image::{Rgba};
use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn to_vec(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

// Make it so we can add vecs to points
impl Add<Vector3> for Point {
    type Output = Point;
    fn add(self, other: Vector3) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;
    fn sub(self, other: Vector3) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point> for Vector3 {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        other - self
    }
}

#[derive(Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
}

pub trait RayShape {

    fn get_center(&self) -> Point;
    fn get_shape(&self) -> ShapeType;
    fn get_color(&self) -> Rgba<u8>;
    fn get_intersection(&self, ray: &Ray) -> Result<(f64, Point), ()>;
}

pub struct Sphere {
    center: Point, 
    radius: f64,
    color: Rgba<u8>,
}

impl RayShape for Sphere {
    fn get_center(&self) -> Point {
        self.center
    }
    fn get_shape(&self) -> ShapeType {
        ShapeType::Sphere
    }
    fn get_intersection(&self, ray: &Ray) -> Result<(f64, Point), ()> {
        let c_minus_r = Vector3::new(self.center.x - ray.get_origin().x, self.center.y - ray.get_origin().y, self.center.z - ray.get_origin().z);
        let inside = c_minus_r.magnitude().sqrt() < self.radius.powf(2.0);
        let t_c = c_minus_r.dot(ray.get_direction()) / ray.get_direction().magnitude();
        if !inside && t_c < 0f64 {
            return Err(());
        }
        let d_squared = (ray.get_origin() + (ray.get_direction() * t_c) - self.center).to_vec().magnitude().powf(2.0);
        if !inside && self.radius.powi(2) < d_squared {
            return Err(());
        }
        // TODO
        Err(())
    }

    fn get_color(&self) -> Rgba<u8> {
        self.color
    }
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, color: Rgba<u8>) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            color,
        }
    }
}

pub struct Plane {
    normal: Vector3,
    point: Point, // TODO fix with proper name
    color: Rgba<u8>,
}

impl RayShape for Plane {
    fn get_center(&self) -> Point {
        self.point
    }
    fn get_shape(&self) -> ShapeType {
        ShapeType::Plane
    }
    fn get_intersection(&self, ray: &Ray) -> Result<(f64, Point), ()> {
        Err(()) 
    }

    fn get_color(&self) -> Rgba<u8> {
        self.color
    }
}

impl Plane {
    pub fn new(x: f64, y: f64, z: f64, d: f64, color: Rgba<u8>) -> Self {
        Self{
            normal: Vector3::new(x, y, z),
            point: Point::new(-d / x, 0.0, 0.0),
            color
        }
    } 
}
