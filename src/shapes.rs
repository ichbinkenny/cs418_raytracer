use std::ops::{Add, Sub};

use crate::bulb::Bulb;
use crate::ray::Ray;
use crate::sun::Sun;
use crate::vector3::Vector3;

pub const MIN_ALLOWED_COLLISION_DISTANCE: f64 = 0.000001;

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShapeType {
    Sphere,
    Plane,
    Triangle,
}

pub trait RayShape {

    fn get_center(&self) -> Point;
    fn get_shape(&self) -> ShapeType;
    fn get_linear_color(&self, objects: &Vec<Box<dyn RayShape>>, bulbs: &Vec<Bulb>, ray: &Ray, sun: &Sun, intersection_point: &Point) -> [f64; 4];
    fn get_color_without_sun(&self) -> [f64; 4];
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Point)>;
    fn get_normal_at_intersection(&self, intersection_point: &Point) -> Vector3;
}

