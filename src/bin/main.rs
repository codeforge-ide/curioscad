use app::{self, Point, Cube, Sphere, Shape, Model, Prompt, process_prompt};

fn main() {
    // Create a new model
    let mut model = Model::new();

    // Create a cube and a sphere
    let cube = Cube::new(Point::new(0.0, 0.0, 0.0), 1.0);
    let sphere = Sphere::new(Point::new(2.0, 0.0, 0.0), 0.5);

    // Add shapes to the model
    model.add_shape(Shape::Cube(cube));
    model.add_shape(Shape::Sphere(sphere));

    // Print the model
    println!("Model: {:#?}", model);

    // Create a prompt and process it
    let prompt = Prompt::new("Create a cylinder");
    let ai_model = process_prompt(&prompt);

    // Print the AI-generated model
    println!("AI Model: {:#?}", ai_model);
}
