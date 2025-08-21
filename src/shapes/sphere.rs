use crate::geometry::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}
