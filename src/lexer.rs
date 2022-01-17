use std::fs::File;
use std::io::BufReader;

pub struct Lexer {
    tokens: Vec<String>,
}

impl Lexer {
    pub fn new(buffer: &BufReader<File>) -> Lexer {
        Lexer { tokens: vec![] }
    }
}
