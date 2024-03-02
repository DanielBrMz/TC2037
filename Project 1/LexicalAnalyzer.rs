impl LexicalAnalyzer {
    pub fn new() -> LexicalAnalyzer {
        LexicalAnalyzer {
            input: String::new(),
            position: 0,
            current_char: None,
        }
    }

    pub fn from_string(input: String) -> LexicalAnalyzer {
        let mut analyzer = LexicalAnalyzer::new();
        analyzer.input = input;
        analyzer.current_char = Some(analyzer.input.chars().nth(0).unwrap());
        analyzer
    }

    pub fn from_file(file_path: &str) -> Result<LexicalAnalyzer, std::io::Error> {
        let input = fs::read_to_string(file_path)?;
        Ok(LexicalAnalyzer::from_string(input))
    }

    pub fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
        } else {
            self.current_char = None;
        }
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

    pub fn integer(&mut self) -> Result<Token, String> {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_numeric() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::INTEGER(result.parse().unwrap()))
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if c.is_numeric() {
                return self.integer();
            }
            if c == '+' {
                self.advance();
                return Ok(Token::PLUS);
            }
            if c == '-' {
                self.advance();
                return Ok(Token::MINUS);
            }
            return Err(format!("Invalid character: {}", c));
        }
        Ok(Token::EOF)
    }
}