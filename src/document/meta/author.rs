use crate::document::meta::Name;

pub struct Author {
    name: Name,
}

impl Author {
    fn new(name: Name) -> Self {
        Self { name }
    }
}