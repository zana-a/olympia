#[derive(Debug)]
pub struct Run {
    options: Vec<Option>,
    bytes: String,
}

impl Run {
    pub fn new(bytes: String) -> Self {
        Self { options: vec![], bytes }
    }

    pub fn apply(&mut self, options: Vec<Option>) {
        self.options = options
    }
}

#[derive(Debug)]
pub enum Option {
    Bold,
    Italic,
    Underline,
    Color(i32),
}