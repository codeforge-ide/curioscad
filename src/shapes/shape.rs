use crate::shapes::cube::Cube;
use crate::shapes::sphere::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere),
}
