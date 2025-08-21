use crate::geometry::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
}

impl Cube {
    pub fn new(center: Point, size: f64) -> Self {
        Self { center, size }
    }
}
