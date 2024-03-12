use std::fs;
use std::io;

#[derive(Debug)]
enum Token {
    Integer(i64),
    Float(f64),
    Assignment,
    Sum,
    Subtract,
    Product,
    Division,
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
    EOF,
}

struct LexicalAnalyzer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl LexicalAnalyzer {
    pub fn new(input: String) -> LexicalAnalyzer {
        let mut analyzer = LexicalAnalyzer {
            input,
            position: 0,
            current_char: None,
        };
        analyzer.current_char = analyzer.input.chars().next();
        analyzer
    }

    pub fn from_file(file_path: &str) -> Result<LexicalAnalyzer, io::Error> {
        let input = fs::read_to_string(file_path)?;
        Ok(LexicalAnalyzer::new(input))
    }

    pub fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn number(&mut self) -> Token {
        let mut number_string = String::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                if c == '.' {
                    if is_float {
                        break; // Second dot in a number, break to avoid parsing error
                    }
                    is_float = true;
                }
                number_string.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(number_string.parse().unwrap())
        } else {
            Token::Integer(number_string.parse().unwrap())
        }
    }

    pub fn identifier(&mut self) -> Result<Token, String> {
        let mut id = String::new();
        let mut valid = true;

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if the identifier starts with a letter or underscore
        if !id.starts_with(char::is_alphabetic) && !id.starts_with('_') {
            valid = false;
        }

        // Check if the identifier contains only letters, digits, or underscores
        if !id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            valid = false;
        }

        if valid {
            Ok(Token::Variable(id))
        } else {
            Err(format!("Unknown identifier: {}", id))
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        while let Some(c) = self.current_char {
            match c {
                _ if c.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                _ if c.is_alphabetic() => return self.identifier(),
                _ if c.is_numeric() || c == '.' => return Ok(self.number()),
                '=' => {
                    self.advance();
                    return Ok(Token::Assignment);
                }
                '+' => {
                    self.advance();
                    return Ok(Token::Sum);
                }
                '-' => {
                    self.advance();
                    return Ok(Token::Subtract);
                }
                '*' => {
                    self.advance();
                    return Ok(Token::Product);
                }
                '/' => {
                    self.advance();
                    return Ok(Token::Division);
                }
                '(' => {
                    self.advance();
                    return Ok(Token::LeftParenthesis);
                }
                ')' => {
                    self.advance();
                    return Ok(Token::RightParenthesis);
                }
                _ => return Err(format!("Invalid character: {}", c)),
            }
        }
        Ok(Token::EOF)
    }
}

fn lexer(filepath: &str) {
    match LexicalAnalyzer::from_file(filepath) {
        Ok(mut analyzer) => {
            loop {
                match analyzer.get_next_token() {
                    Ok(token) => {
                        match &token {
                            Token::EOF => break,
                            Token::Variable(var) => println!("Token: {} Type: {}", var, token_type(&token)),
                            Token::Integer(int) => println!("Token: {} Type: {}", int, token_type(&token)),
                            _ => println!("Type: {} Token: {:?}", token_type(&token), token),
                        }
                    },
                    Err(e) => {
                        println!("Token: {} Type: unknown", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("Failed to read file: {}", e),
    }
}

fn token_type(token: &Token) -> &str {
    match token {
        Token::Integer(_) => "integer",
        Token::Float(_) => "float",
        Token::Assignment => "=",
        Token::Sum => "+",
        Token::Subtract => "-",
        Token::Product => "*",
        Token::Division => "/",
        Token::Variable(_) => "variable",
        Token::LeftParenthesis => "(",
        Token::RightParenthesis => ")",
        Token::EOF => "end of file",
    }
}

fn main() {
    let filepath = "expressions.txt";
    lexer(filepath);
}