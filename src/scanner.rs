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
pub enum ScannerError {
    UnexpectedToken(usize, usize),
    NumberLiteralParsingError(usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct TokenMetadata {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

/// Scanner is an iterator object over a vector of characters making up the code of the script
struct Scanner {
    /// A vector of characters containing the code of the script
    code: Vec<char>,

    /// Current position in the vector
    current: usize,

    /// Current line of code
    line: usize,

    /// Current column of code
    column: usize,
}

/// Make the scanner object into an iterator over a 2-character window with next being an Option<char>
impl Iterator for Scanner {
    type Item = (char, Option<char>);

    /// Get next window of the current character and an Option<char> of the next character
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Is there a nicer way to dereference Option<&char> to Option<char> ?
        if let Some(&curr) = self.code.get(self.current) {
            self.advance();

            if let Some(&next) = self.code.get(self.current) {
                Some((curr, Some(next)))
            } else {
                Some((curr, None))
            }
        } else {
            None
        }
    }
}

impl Scanner {
    /// Create new scanner based on a &str of code
    fn new(code: &str) -> Self {
        Scanner {
            code: code.chars().collect(),
            current: 0,
            line: 1,
            column: 0,
        }
    }

    /// Reset scanner to the start of the code
    fn reset(&mut self) {
        self.current = 0;
        self.line = 1;
        self.column = 0;
    }

    /// Advance one step without getting the iterator output from self.next()
    fn advance(&mut self) {
        self.current += 1;
        self.column += 1;
    }

    /// Newline and return column to zero
    fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    /// Parse all tokens from the underlaying vector of characters
    fn tokens(&mut self) -> Result<Vec<TokenMetadata>, ScannerError> {
        self.reset();

        let mut tokens: Vec<TokenMetadata> = vec![];

        while let Some((curr, next)) = self.next() {
            let (line, column) = (self.line, self.column);

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
                // Consume iterator until newline
                for (_, next) in self.by_ref() {
                    if next == Some('\n') {
                        break;
                    }
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
                    line,
                    column,
                });
                continue;
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
                    line,
                    column,
                });

                // Advance one extra step if two characters
                if next == Some('=') {
                    self.advance();
                }
                continue;
            }

            // Keywords and identifiers
            if curr.is_alphabetic() {
                let mut identifier = String::from(curr);
                if next.is_some_and(|n| n.is_alphanumeric()) {
                    for (curr, next) in self.by_ref() {
                        identifier.push(curr);
                        if !next.is_some_and(|n|n.is_alphanumeric()) {
                            break;
                        }
                    }
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
                continue;
            }

            // Number litterals
            if curr.is_numeric() {
                let mut number = String::from(curr);
                if next.is_some_and(|n| n.is_numeric() || n == '.') {
                    for (curr, next) in self.by_ref() {
                        number.push(curr);
                        if !next.is_some_and(|n|n.is_numeric() || n == '.') {
                            break;
                        }
                    }
                }

                if let Ok(number) = number.parse() {
                    tokens.push(TokenMetadata {
                        token: Token::Number(number),
                        line,
                        column,
                    });
                }
                else {
                    return Err(ScannerError::NumberLiteralParsingError(line, column));
                }
                continue;
            }

            // String Litterals
            if curr == '"' {
                let (line, column) = (self.line, self.column);

                // TODO: Add support for escape characters like '\n', '\\' or '\"' 
                // TODO: Error on newline in string

                let mut string = String::new();
                while let Some((curr, next)) = self.next() {
                    string.push(curr);
                    if next == Some('"') {
                        self.advance();
                        break;
                    }
                }

                tokens.push(TokenMetadata {
                    token: Token::String(string),
                    line,
                    column,
                });

                continue;
            }

            return Err(ScannerError::UnexpectedToken(line, column));
        }

        tokens.push(TokenMetadata {
            token: Token::Eof,
            line: self.line,
            column: self.column + 1,
        });
        Ok(tokens)
    }
}

/// Scannable trait can be put on enything that can be converted to a string of code
pub trait Scannable {
    fn tokens(&self) -> Result<Vec<TokenMetadata>, ScannerError>;
}

/// Implement scannable for &str
impl Scannable for &str {
    /// Scan a string of code for tokens
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
