use crate::shapes::Point;

pub struct Bulb {
    location: Point,
    color: [f64; 4],
}

impl Bulb {
    pub fn new(location: Point, color: [f64; 4]) -> Self {
        Self {
            location,
            color
        }
    }

    pub fn get_light(&self, object_location: &Point) -> [f64; 4] {
        // have light decrease with eq 1 / distance^2
        let distance = self.location.to_vec() - object_location.to_vec();
        let light_factor = 1.0 / (distance.magnitude() * distance.magnitude());
        self.color.map(|c| c * light_factor)
    }
}