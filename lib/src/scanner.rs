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
        match self.source.chars().nth(self.start) {
            Some('a') => self.check_keyword(1, "nd", TokenType::And),
            Some('c') => self.check_keyword(1, "lass", TokenType::Class),
            Some('e') => self.check_keyword(1, "lse", TokenType::Else),
            Some('i') => self.check_keyword(1, "f", TokenType::If),
            Some('n') => self.check_keyword(1, "il", TokenType::Nil),
            Some('o') => self.check_keyword(1, "r", TokenType::Or),
            Some('p') => self.check_keyword(1, "rint", TokenType::Print),
            Some('r') => self.check_keyword(1, "eturn", TokenType::Return),
            Some('s') => self.check_keyword(1, "uper", TokenType::Super),
            Some('v') => self.check_keyword(1, "ar", TokenType::Var),
            Some('w') => self.check_keyword(1, "hile", TokenType::While),
            Some('f') => match self.source.chars().nth(self.start + 1) {
                Some('a') => self.check_keyword(2, "lse", TokenType::False),
                Some('o') => self.check_keyword(2, "r", TokenType::For),
                Some('u') => self.check_keyword(2, "n", TokenType::Fun),
                _ => TokenType::Identifier,
            },
            Some('t') => match self.source.chars().nth(self.start + 1) {
                Some('h') => self.check_keyword(2, "is", TokenType::This),
                Some('r') => self.check_keyword(2, "ue", TokenType::True),
                _ => TokenType::Identifier,
            },
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        let len = rest.len();
        let slice = self
            .source
            .get(self.start + start..self.start + start + len);

        match slice {
            Some(s) if s == rest => token_type,
            _ => TokenType::Identifier,
        }
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
