pub mod ai;
pub mod geometry;
pub mod model;
pub mod shapes;

pub use ai::prompt::{process_prompt, Prompt};
pub use geometry::point::Point;
pub use geometry::vector::Vector;
pub use model::Model;
pub use shapes::cube::Cube;
pub use shapes::shape::Shape;
pub use shapes::sphere::Sphere;
