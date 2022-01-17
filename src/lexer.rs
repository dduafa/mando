use crate::constants;
use crate::token;
use constants::{DOUBLE_QUOTE, ESCAPE_CHARACTER, SEMI_COLON, SINGLE_QUOTE, SPACE_CHARACTER};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use token::Token;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    file: &'a File,
    state: LexerState,
    lexeme: String,
    _string_char: Option<char>,
    previous_char: Option<char>,
}

#[derive(Debug)]
pub enum LexerState {
    IsIdle,
    IsReadingIdentifier,
    IsReadingNumber,
    IsReadingString,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a File) -> Self {
        Self {
            tokens: vec![],
            file,
            state: LexerState::IsIdle,
            lexeme: "".to_owned(),
            _string_char: None,
            previous_char: None,
        }
    }

    fn generate_tokens(&mut self) {
        let buffer = BufReader::new(self.file);
        for (index, line) in buffer.lines().enumerate() {
            if let Ok(data) = line {
                for character in data.chars().collect::<Vec<char>>() {
                    match &self.state {
                        LexerState::IsIdle => {
                            if character.is_numeric() {
                                self.lexeme.push(character);
                                self.state = LexerState::IsReadingNumber
                            } else if character.is_alphabetic() {
                                self.lexeme.push(character);
                                self.state = LexerState::IsReadingIdentifier
                            } else if character == DOUBLE_QUOTE || character == SINGLE_QUOTE {
                                self.lexeme.push(character);
                                self._string_char = Some(character);
                                self.state = LexerState::IsReadingString
                            }
                        }
                        LexerState::IsReadingIdentifier => {
                            if character == SPACE_CHARACTER || character == SEMI_COLON {
                                self.tokens.push(self.create_token(index));
                                self.state = LexerState::IsIdle;
                                self.lexeme.clear();
                            } else {
                                self.lexeme.push(character);
                            }
                        }
                        LexerState::IsReadingNumber => {
                            if character == SPACE_CHARACTER || character == SEMI_COLON {
                                self.tokens.push(self.create_token(index));
                                self.state = LexerState::IsIdle;
                                self.lexeme.clear();
                            } else {
                                self.lexeme.push(character);
                            }
                        }
                        LexerState::IsReadingString => {
                            self.lexeme.push(character);
                            if character == self._string_char.unwrap()
                                && self.previous_char.unwrap() != ESCAPE_CHARACTER
                            {
                                self.tokens.push(self.create_token(index));
                                self.state = LexerState::IsIdle;
                                self.lexeme.clear();
                            }
                        }
                    }
                    self.previous_char = Some(character);
                }
            }
        }
    }

    fn create_token(&self, line: usize) -> Token {
        let temp = Token::new(&self.state, self.lexeme.clone());
        if let Err(err) = temp {
            eprintln!("mando error: {}", err);
            eprintln!("line: {}", line + 1);
            process::exit(1);
        }
        temp.unwrap()
    }
    pub fn get_tokens(&mut self) -> &Vec<Token> {
        self.generate_tokens();

        return &self.tokens;
    }
}
