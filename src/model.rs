use crate::shapes::shape::Shape;

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub shapes: Vec<Shape>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
}
