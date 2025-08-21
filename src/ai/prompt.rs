use crate::model::Model;

pub struct Prompt {
    pub text: String,
}

impl Prompt {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

pub fn process_prompt(prompt: &Prompt) -> Model {
    // In the future, this function will use an AI model to generate a 3D model from the prompt.
    // For now, it returns an empty model.
    println!("Processing prompt: {}", prompt.text);
    Model::new()
}
