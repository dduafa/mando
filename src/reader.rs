use std::fs::File;

pub struct TranslationUnit {
    content: File,
}

impl TranslationUnit {
    pub fn new(filename: &String) -> Result<Self, &'static str> {
        if let Ok(file) = File::open(filename) {
            return Ok(Self { content: file });
        }
        Err("Unable to open file")
    }

    pub fn file_content(&self) -> &File {
        return &self.content;
    }
}
