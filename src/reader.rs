use std::fs::File;
use std::io::BufReader;

pub struct TranslationUnit {
    content: BufReader<File>,
}

impl TranslationUnit {
    pub fn new(filename: &String) -> Result<TranslationUnit, &'static str> {
        if let Ok(file) = File::open(filename) {
            let buffer = BufReader::new(file);
            return Ok(TranslationUnit { content: buffer });
        }
        Err("Unable to open file")
    }

    pub fn file_content(&self) -> &BufReader<File> {
        return &self.content;
    }
}
