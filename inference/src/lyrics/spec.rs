pub struct LyricSpec {
    pub language: String,
    pub description: String,
}

impl LyricSpec {
    pub fn empty() -> Self {
        Self {
            language: String::new(),
            description: String::new(),
        }
    }
}
