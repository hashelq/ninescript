use crate::ast::Statement;

pub struct Processor {
    source: Vec<Statement>,
    position: usize
}

impl Processor {
    pub fn new(source: Vec<Statement>) -> Self {
        Self {
            source,
            position: 0
        }
    }

    pub fn ir() -> String {

    }
}
