pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        let c = if let Some(c) = self.advance() {
            c
        } else {
            return self.make_eof();
        };

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let next_equal = self.match_char('=');
                if next_equal {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                let next_equal = self.match_char('=');
                if next_equal {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                let next_equal = self.match_char('=');
                if next_equal {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                let next_equal = self.match_char('=');
                if next_equal {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn match_char(&mut self, c: char) -> bool {
        if let Some(current) = self.source.chars().next() {
            if current != c {
                return false;
            }

            self.current += 1;

            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if let Some('/') = self.peek_next() {
                        loop {
                            match self.peek() {
                                Some('\n') => break,
                                Some(_) => {
                                    self.advance();
                                }
                                None => break,
                            }
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme = &self.source[self.start..self.current];
        Token {
            token_type,
            lexeme,
            line: self.line,
        }
    }

    fn make_eof(&self) -> Token {
        Token {
            token_type: TokenType::EOF,
            lexeme: "\0",
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        loop {
            match self.peek() {
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    return self.error_token("Unterminated string.");
                }
            }
        }

        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }

        let has_fraction = matches!(self.peek(), Some('.'));
        let next_digit = matches!(self.peek_next(), Some('0'..='9'));

        if has_fraction && next_digit {
            self.advance();

            while let Some('0'..='9') = self.peek() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while let Some('a'..='z') | Some('A'..='Z') | Some('_') = self.peek() {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        TokenType::Identifier
    }

    fn error_token(&self, message: &'a str) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    EOF,
}
