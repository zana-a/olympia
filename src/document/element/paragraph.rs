use crate::Run;

#[derive(Debug)]
pub struct Paragraph {
    runs: Option<Vec<Run>>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { runs: None }
    }

    pub fn insert_run(&mut self, run: Run) {
        match &mut self.runs {
            Some(vec) => vec.push(run),
            None => {
                self.runs.insert(vec![run]);
            }
        }
    }
}