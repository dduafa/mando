use crate::lexer;
use lexer::LexerState;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
    value: String,
    t_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Nil,
    Int32,
    Int64,
    Str,
    Mutable,
    String,
    Number,
    Identifier,
}

impl Token {
    pub fn new(lexer_state: &LexerState, value: String) -> Result<Self, String> {
        let mut token = Self {
            t_type: TokenType::Nil,
            value: value,
        };
        let token_type = token.get_token_type(lexer_state);

        if token_type.is_ok() {
            token.t_type = token_type.unwrap();
            return Ok(token);
        }

        return Err(token_type.unwrap_err());
    }

    fn get_type_token_type_from_value(&self, value: &str) -> TokenType {
        match value {
            "i32" => TokenType::Int32,
            "i64" => TokenType::Int64,
            _ => TokenType::Str,
        }
    }

    fn get_token_type(&self, lexer_state: &LexerState) -> Result<TokenType, String> {
        let supported_types: Vec<&str> = vec!["i64", "i32", "string"];
        match lexer_state {
            LexerState::IsReadingIdentifier => {
                if supported_types.contains(&self.value.as_str()) {
                    return Ok(self.get_type_token_type_from_value(self.value.as_str()));
                } else if &self.value == "mut" {
                    return Ok(TokenType::Mutable);
                }

                let re = Regex::new(r"^[_A-Za-z](?:\w)*$").unwrap();
                if re.is_match(self.value.as_str()) {
                    return Ok(TokenType::Identifier);
                }
                return Err(format!("{} is an invalid identifier", self.value));
            }
            LexerState::IsReadingNumber => Ok(TokenType::Number),
            LexerState::IsReadingString => Ok(TokenType::String),
            _ => panic!("Invalid lexer state for token type"),
        }
    }
}
