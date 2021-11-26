pub struct Name {
    first: String,
    last: String,
}

impl Name {
    fn new(first: String, last: String) -> Self {
        Name { first, last }
    }
}