#[derive(Debug, PartialEq)]
pub enum Token {
    // Single character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,

    Eof,
}

#[derive(Debug)]
pub enum ScannerError {}

#[derive(Debug, PartialEq)]
pub struct TokenMetadata {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub struct Scanner {
    code: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    fn new(code: &str) -> Self {
        Scanner {
            code: code.chars().collect(),
            current: 0,
            line: 1,
            column: 0,
        }
    }

    fn reset(&mut self) {
        self.current = 0;
        self.line = 1;
        self.column = 0;
    }

    fn advance(&mut self) {
        // -> Option<char, Option<char>> {
        self.current += 1;
        self.column += 1;
    }

    fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    fn current(&self) -> Option<char> {
        if let Some(&c) = self.code.get(self.current) {
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        if let Some(&c) = self.code.get(self.current + 1) {
            Some(c)
        } else {
            None
        }
    }

    pub fn tokens(&mut self) -> Result<Vec<TokenMetadata>, ScannerError> {
        self.reset();

        let mut tokens: Vec<TokenMetadata> = vec![];

        while let Some(curr) = self.current() {
            let mut next = self.peek();

            self.advance();

            // Newline
            if curr == '\n' {
                self.newline();
                continue;
            }

            // Whitespace
            if curr.is_whitespace() {
                continue;
            }

            // Skip comments
            if curr == '/' && next == Some('/') {
                while next.is_some_and(|n| n != '\n') {
                    next = self.peek();
                    self.advance();
                }
                continue;
            }

            // Single character
            if let Some(token) = match curr {
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                ',' => Some(Token::Comma),
                '.' => Some(Token::Dot),
                '-' => Some(Token::Minus),
                '+' => Some(Token::Plus),
                ';' => Some(Token::Semicolon),
                '/' => Some(Token::Slash),
                '*' => Some(Token::Star),
                _ => None,
            } {
                tokens.push(TokenMetadata {
                    token,
                    line: self.line,
                    column: self.column,
                });
            }

            // One or two character
            if let Some(token) = match (curr, next) {
                ('!', Some('=')) => Some(Token::BangEqual),
                ('!', _) => Some(Token::Bang),
                ('=', Some('=')) => Some(Token::EqualEqual),
                ('=', _) => Some(Token::Equal),
                ('>', Some('=')) => Some(Token::GreaterEqual),
                ('>', _) => Some(Token::Greater),
                ('<', Some('=')) => Some(Token::LessEqual),
                ('<', _) => Some(Token::Less),
                _ => None,
            } {
                tokens.push(TokenMetadata {
                    token,
                    line: self.line,
                    column: self.column,
                });

                // Advance one extra step if two characters
                if next == Some('=') {
                    self.advance();
                }
            }

            // Keywords and identifiers
            if curr.is_alphabetic() {
                let (line, column) = (self.line, self.column);

                let mut identifier = String::from(curr);
                while next.is_some_and(|n| n.is_alphanumeric()) {
                    identifier.push(next.unwrap());
                    next = self.peek();
                    self.advance();
                }

                let token = match identifier.as_str() {
                    "and" => Token::And,
                    "class" => Token::Class,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "fn" => Token::Fn,
                    "for" => Token::For,
                    "if" => Token::If,
                    "null" => Token::Null,
                    "or" => Token::Or,
                    "print" => Token::Print,
                    "return" => Token::Return,
                    "super" => Token::Super,
                    "this" => Token::This,
                    "true" => Token::True,
                    "let" => Token::Let,
                    "while" => Token::While,
                    ident => Token::Identifier(String::from(ident)),
                };

                tokens.push(TokenMetadata {
                    token,
                    line,
                    column,
                });
            }

            // Numbers
            if curr.is_numeric() {
                let (line, column) = (self.line, self.column);

                let mut number = String::from(curr);
                while next.is_some_and(|n| n.is_numeric() || n == '.') {
                    number.push(next.unwrap());
                    next = self.peek();
                    self.advance();
                }

                tokens.push(TokenMetadata {
                    token: Token::Number(number.parse().unwrap()),
                    line,
                    column,
                });
            }

            // String Litterals
            if curr == '"' {
                let (line, column) = (self.line, self.column);

                let mut string = String::new();
                while next.is_some_and(|n| n != '"') {
                    string.push(next.unwrap());
                    next = self.peek();
                    self.advance();
                }

                // Skip final quote
                self.advance();

                tokens.push(TokenMetadata {
                    token: Token::String(string),
                    line,
                    column,
                });
            }
        }

        tokens.push(TokenMetadata {
            token: Token::Eof,
            line: self.line,
            column: self.column + 1,
        });
        Ok(tokens)
    }
}

pub trait Scannable {
    fn tokens(&self) -> Result<Vec<TokenMetadata>, ScannerError>;
}

impl Scannable for &str {
    fn tokens(&self) -> Result<Vec<TokenMetadata>, ScannerError> {
        let mut scanner = Scanner::new(self);
        scanner.tokens()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokens() -> Vec<TokenMetadata> {
        r#"
// Single character tokens
(){},.-+;/*
// One or two character tokens
! != = == > >= < <=
// Literals
let greeting = "hello";
let fraction = 0.5;
let integer = 123;
// Keywords
and class else false fn for if null or print return super this true let while
"#
            .tokens()
            .unwrap()
    }

    #[test]
    fn single_character_tokens() {
        let tokens = test_tokens();

        assert_eq!(tokens[0],  TokenMetadata {token: Token::LeftParenthesis,  line: 3, column:  1});
        assert_eq!(tokens[1],  TokenMetadata {token: Token::RightParenthesis, line: 3, column:  2});
        assert_eq!(tokens[2],  TokenMetadata {token: Token::LeftBrace,        line: 3, column:  3});
        assert_eq!(tokens[3],  TokenMetadata {token: Token::RightBrace,       line: 3, column:  4});
        assert_eq!(tokens[4],  TokenMetadata {token: Token::Comma,            line: 3, column:  5});
        assert_eq!(tokens[5],  TokenMetadata {token: Token::Dot,              line: 3, column:  6});
        assert_eq!(tokens[6],  TokenMetadata {token: Token::Minus,            line: 3, column:  7});
        assert_eq!(tokens[7],  TokenMetadata {token: Token::Plus,             line: 3, column:  8});
        assert_eq!(tokens[8],  TokenMetadata {token: Token::Semicolon,        line: 3, column:  9});
        assert_eq!(tokens[9],  TokenMetadata {token: Token::Slash,            line: 3, column: 10});
        assert_eq!(tokens[10], TokenMetadata {token: Token::Star,             line: 3, column: 11});
    }

    #[test]
    fn one_or_two_character_tokens() {
        let tokens = test_tokens();

        assert_eq!(tokens[11], TokenMetadata {token: Token::Bang,         line: 5, column:  1});
        assert_eq!(tokens[12], TokenMetadata {token: Token::BangEqual,    line: 5, column:  3});
        assert_eq!(tokens[13], TokenMetadata {token: Token::Equal,        line: 5, column:  6});
        assert_eq!(tokens[14], TokenMetadata {token: Token::EqualEqual,   line: 5, column:  8});
        assert_eq!(tokens[15], TokenMetadata {token: Token::Greater,      line: 5, column: 11});
        assert_eq!(tokens[16], TokenMetadata {token: Token::GreaterEqual, line: 5, column: 13});
        assert_eq!(tokens[17], TokenMetadata {token: Token::Less,         line: 5, column: 16});
        assert_eq!(tokens[18], TokenMetadata {token: Token::LessEqual,    line: 5, column: 18});
    }

    #[test]
    fn literals() {
        let tokens = test_tokens();

        // Identifiers
        assert_eq!(tokens[20], TokenMetadata {token: Token::Identifier(String::from("greeting")), line: 7, column: 5});
        assert_eq!(tokens[25], TokenMetadata {token: Token::Identifier(String::from("fraction")), line: 8, column: 5});
        assert_eq!(tokens[30], TokenMetadata {token: Token::Identifier(String::from("integer")),  line: 9, column: 5});

        // String literal
        assert_eq!(tokens[22], TokenMetadata {token: Token::String(String::from("hello")), line: 7, column: 16});

        // Numbers
        assert_eq!(tokens[27], TokenMetadata {token: Token::Number(0.5f64), line: 8, column: 16});
        assert_eq!(tokens[32], TokenMetadata {token: Token::Number(123f64), line: 9, column: 15});
    }

    #[test]
    fn keywords() {
        let tokens = test_tokens();

        // Identifiers
        assert_eq!(tokens[34], TokenMetadata {token: Token::And,    line: 11, column: 1});
        assert_eq!(tokens[35], TokenMetadata {token: Token::Class,  line: 11, column: 5});
        assert_eq!(tokens[36], TokenMetadata {token: Token::Else,   line: 11, column: 11});
        assert_eq!(tokens[37], TokenMetadata {token: Token::False,  line: 11, column: 16});
        assert_eq!(tokens[38], TokenMetadata {token: Token::Fn,     line: 11, column: 22});
        assert_eq!(tokens[39], TokenMetadata {token: Token::For,    line: 11, column: 25});
        assert_eq!(tokens[40], TokenMetadata {token: Token::If,     line: 11, column: 29});
        assert_eq!(tokens[41], TokenMetadata {token: Token::Null,   line: 11, column: 32});
        assert_eq!(tokens[42], TokenMetadata {token: Token::Or,     line: 11, column: 37});
        assert_eq!(tokens[43], TokenMetadata {token: Token::Print,  line: 11, column: 40});
        assert_eq!(tokens[44], TokenMetadata {token: Token::Return, line: 11, column: 46});
        assert_eq!(tokens[45], TokenMetadata {token: Token::Super,  line: 11, column: 53});
        assert_eq!(tokens[46], TokenMetadata {token: Token::This,   line: 11, column: 59});
        assert_eq!(tokens[47], TokenMetadata {token: Token::True,   line: 11, column: 64});
        assert_eq!(tokens[48], TokenMetadata {token: Token::Let,    line: 11, column: 69});
        assert_eq!(tokens[49], TokenMetadata {token: Token::While,  line: 11, column: 73});
    }

    #[test]
    fn eof() {
        let tokens = test_tokens();

        assert_eq!(tokens.last(), Some(&TokenMetadata {token: Token::Eof, line: 12, column: 1}));
    }

}
