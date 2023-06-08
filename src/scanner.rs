use crate::token::*;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedToken(Position),
    NumberLiteralParsingError(Position),
    UnterminatedMultilineComment(Position),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(position) => write!(f, "Unexpected token at {}", position),
            Self::NumberLiteralParsingError(position) => write!(f, "Error parsing number at {}", position),
            Self::UnterminatedMultilineComment(position) => write!(f, "Unterminated multiline comment at {}", position),
        }
    }
}

impl Error for ScannerError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}", self.line, self.column)
    }
}

#[derive(Debug)]
/// Scanner is an iterator object over a vector of characters making up the code of the script
struct Scanner {
    /// A vector of characters containing the code of the script
    code: Vec<char>,

    /// Current position in the vector
    current: usize,

    /// Current position in the code file (line, column)
    position: Position,
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
            position: Position { line: 0, column: 0 }
        }
    }

    /// Reset scanner to the start of the code
    fn reset(&mut self) {
        self.current = 0;
        self.position.line = 1;
        self.position.column = 0;
    }

    /// Advance one step without getting the iterator output from self.next()
    fn advance(&mut self) {
        self.current += 1;
        self.position.column += 1;
    }

    /// Newline and return column to zero
    fn newline(&mut self) {
        self.position.line += 1;
        self.position.column = 0;
    }

    /// Scan a single line comment from current position
    fn scan_line_comment(&mut self) -> Result<(),ScannerError> {
        // Consume iterator until newline
        for (_, next) in self.by_ref() {
            if next == Some('\n') {
                break;
            }
        }
        Ok(())
    }

    fn scan_multiline_comment(&mut self) -> Result<(), ScannerError> {
        // Consume iterator until newline
        while let Some((curr, next)) = self.next() {
            match (curr, next) {
                ('\n', _) => self.newline(),
                ('*', Some('/')) => {self.advance(); break},
                (_, None) => return Err(ScannerError::UnterminatedMultilineComment(self.position)),
                _ => {}
            }
        }
        Ok(())
    }

    /// Scan a number literal from current position
    fn scan_number_literal(&mut self, initial: char) -> Result<Token, ScannerError> {
        let position = self.position;

        let mut number = String::from(initial);
        for (curr, next) in self.by_ref() {
            number.push(curr);
            if !next.is_some_and(|n|n.is_numeric() || n == '.') {
                break;
            }
        }

        if let Ok(number) = number.parse() {
            Ok(Token::Number(number))
        }
        else {
            Err(ScannerError::NumberLiteralParsingError(position))
        }
    }

    /// Scan a string literal from current position
    fn scan_string_literal(&mut self) -> Result<Token, ScannerError> {
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

        Ok(Token::String(string))
    }

    /// Scan a keyword or identifier from current position
    fn scan_keyword_or_identifier(&mut self, initial: char) -> Result<Token, ScannerError>{
        let mut identifier = String::from(initial);
        for (curr, next) in self.by_ref() {
            identifier.push(curr);
            if !next.is_some_and(|n|n.is_alphanumeric()) {
                break;
            }
        }

        // This is the list of reserved keywords
        Ok(match identifier.as_str() {
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "fn" => Token::Fn,
            "for" => Token::For,
            "if" => Token::If,
            "null" => Token::Null,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "let" => Token::Let,
            "while" => Token::While,
            ident => Token::Identifier(String::from(ident)),
        })
    }

    /// Parse all tokens from the underlaying vector of characters
    fn tokens(&mut self) -> Result<Vec<TokenMetadata>, ScannerError> {
        self.reset();

        let mut tokens: Vec<TokenMetadata> = vec![];

        while let Some((curr, next)) = self.next() {
            let position = self.position;

            let token = match (curr, next) {
                // Newline
                ('\n', _ )                  => {self.newline(); None},

                // Whitespace
                _ if curr.is_whitespace()   => None,

                // Comments
                ('/', Some('/')) => {self.scan_line_comment()?; None},
                ('/', Some('*')) => {self.scan_multiline_comment()?; None}

                // Single character tokens
                ('(', _) => Some(Token::LeftParenthesis),
                (')', _) => Some(Token::RightParenthesis),
                ('{', _) => Some(Token::LeftBrace),
                ('}', _) => Some(Token::RightBrace),
                (',', _) => Some(Token::Comma),
                ('.', _) => Some(Token::Dot),
                ('-', _) => Some(Token::Minus),
                ('+', _) => Some(Token::Plus),
                (':', _) => Some(Token::Colon),
                (';', _) => Some(Token::Semicolon),
                ('/', _) => Some(Token::Slash),
                ('*', _) => Some(Token::Star),
                ('?', _) => Some(Token::Question),

                // One or two character tokens
                ('!', Some('=')) => { self.advance(); Some(Token::BangEqual)},
                ('=', Some('=')) => { self.advance(); Some(Token::EqualEqual)},
                ('>', Some('=')) => { self.advance(); Some(Token::GreaterEqual)},
                ('<', Some('=')) => { self.advance(); Some(Token::LessEqual)},
                ('&', Some('&')) => { self.advance(); Some(Token::And)},
                ('|', Some('|')) => { self.advance(); Some(Token::Or)},
                ('!', _) => Some(Token::Bang),
                ('=', _) => Some(Token::Equal),
                ('>', _) => Some(Token::Greater),
                ('<', _) => Some(Token::Less),

                // Keywords and identifiers
                _ if curr.is_alphabetic() => Some(self.scan_keyword_or_identifier(curr)?),

                // Number literals
                _ if curr.is_numeric() => Some(self.scan_number_literal(curr)?),

                // String literals
                ('"', _) => Some(self.scan_string_literal()?),

                // Unexpected -> Error
                _ => return Err(ScannerError::UnexpectedToken(position)),
            };

            if let Some(token) = token {
                tokens.push(TokenMetadata { token, position });
            }
        }

        // Add Eof-token
        self.advance();
        tokens.push(TokenMetadata {
            token: Token::Eof,
            position: self.position,
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
        std::fs::read_to_string("test.lhscript")
            .unwrap()
            .as_str().tokens()
            .unwrap()
    }

    #[test]
    fn single_character_tokens() {
        let tokens = test_tokens();

        assert_eq!(tokens[0],  TokenMetadata {token: Token::LeftParenthesis,  position: Position {line: 2, column:  1}});
        assert_eq!(tokens[1],  TokenMetadata {token: Token::RightParenthesis, position: Position {line: 2, column:  2}});
        assert_eq!(tokens[2],  TokenMetadata {token: Token::LeftBrace,        position: Position {line: 2, column:  3}});
        assert_eq!(tokens[3],  TokenMetadata {token: Token::RightBrace,       position: Position {line: 2, column:  4}});
        assert_eq!(tokens[4],  TokenMetadata {token: Token::Comma,            position: Position {line: 2, column:  5}});
        assert_eq!(tokens[5],  TokenMetadata {token: Token::Dot,              position: Position {line: 2, column:  6}});
        assert_eq!(tokens[6],  TokenMetadata {token: Token::Minus,            position: Position {line: 2, column:  7}});
        assert_eq!(tokens[7],  TokenMetadata {token: Token::Plus,             position: Position {line: 2, column:  8}});
        assert_eq!(tokens[8],  TokenMetadata {token: Token::Colon,            position: Position {line: 2, column:  9}});
        assert_eq!(tokens[9],  TokenMetadata {token: Token::Semicolon,        position: Position {line: 2, column: 10}});
        assert_eq!(tokens[10], TokenMetadata {token: Token::Star,             position: Position {line: 2, column: 11}});
        assert_eq!(tokens[11], TokenMetadata {token: Token::Slash,            position: Position {line: 2, column: 12}});
        assert_eq!(tokens[12], TokenMetadata {token: Token::Question,         position: Position {line: 2, column: 13}});
    }

    #[test]
    fn one_or_two_character_tokens() {
        let tokens = test_tokens();

        assert_eq!(tokens[13], TokenMetadata {token: Token::Bang,         position: Position {line: 4, column:  1}});
        assert_eq!(tokens[14], TokenMetadata {token: Token::BangEqual,    position: Position {line: 4, column:  3}});
        assert_eq!(tokens[15], TokenMetadata {token: Token::Equal,        position: Position {line: 4, column:  6}});
        assert_eq!(tokens[16], TokenMetadata {token: Token::EqualEqual,   position: Position {line: 4, column:  8}});
        assert_eq!(tokens[17], TokenMetadata {token: Token::Greater,      position: Position {line: 4, column: 11}});
        assert_eq!(tokens[18], TokenMetadata {token: Token::GreaterEqual, position: Position {line: 4, column: 13}});
        assert_eq!(tokens[19], TokenMetadata {token: Token::Less,         position: Position {line: 4, column: 16}});
        assert_eq!(tokens[20], TokenMetadata {token: Token::LessEqual,    position: Position {line: 4, column: 18}});
        assert_eq!(tokens[21], TokenMetadata {token: Token::And,          position: Position {line: 4, column: 21}});
        assert_eq!(tokens[22], TokenMetadata {token: Token::Or,           position: Position {line: 4, column: 24}});
    }

    #[test]
    fn literals() {
        let tokens = test_tokens();

        // Identifiers
        assert_eq!(tokens[24], TokenMetadata {token: Token::Identifier(String::from("greeting")), position: Position {line: 6, column: 5}});
        assert_eq!(tokens[29], TokenMetadata {token: Token::Identifier(String::from("fraction")), position: Position {line: 7, column: 5}});
        assert_eq!(tokens[34], TokenMetadata {token: Token::Identifier(String::from("integer")),  position: Position {line: 8, column: 5}});

        // String literal
        assert_eq!(tokens[26], TokenMetadata {token: Token::String(String::from("hello")), position: Position {line: 6, column: 16}});

        // Numbers
        assert_eq!(tokens[31], TokenMetadata {token: Token::Number(0.5f64), position: Position {line: 7, column: 16}});
        assert_eq!(tokens[36], TokenMetadata {token: Token::Number(123f64), position: Position {line: 8, column: 15}});
    }

    #[test]
    fn keywords() {
        let tokens = test_tokens();

        // Identifiers
        assert_eq!(tokens[38], TokenMetadata {token: Token::Class,  position: Position {line: 11, column: 1}});
        assert_eq!(tokens[39], TokenMetadata {token: Token::Else,   position: Position {line: 11, column: 7}});
        assert_eq!(tokens[40], TokenMetadata {token: Token::False,  position: Position {line: 11, column: 12}});
        assert_eq!(tokens[41], TokenMetadata {token: Token::Fn,     position: Position {line: 11, column: 18}});
        assert_eq!(tokens[42], TokenMetadata {token: Token::For,    position: Position {line: 11, column: 21}});
        assert_eq!(tokens[43], TokenMetadata {token: Token::If,     position: Position {line: 11, column: 25}});
        assert_eq!(tokens[44], TokenMetadata {token: Token::Null,   position: Position {line: 11, column: 28}});
        assert_eq!(tokens[45], TokenMetadata {token: Token::Print,  position: Position {line: 11, column: 33}});
        assert_eq!(tokens[46], TokenMetadata {token: Token::Return, position: Position {line: 11, column: 39}});
        assert_eq!(tokens[47], TokenMetadata {token: Token::Super,  position: Position {line: 11, column: 46}});
        assert_eq!(tokens[48], TokenMetadata {token: Token::This,   position: Position {line: 11, column: 52}});
        assert_eq!(tokens[49], TokenMetadata {token: Token::True,   position: Position {line: 11, column: 57}});
        assert_eq!(tokens[50], TokenMetadata {token: Token::Let,    position: Position {line: 11, column: 62}});
        assert_eq!(tokens[51], TokenMetadata {token: Token::While,  position: Position {line: 11, column: 66}});
    }

    #[test]
    fn eof() {
        let tokens = test_tokens();

        assert_eq!(tokens.last(), Some(&TokenMetadata {token: Token::Eof, position: Position {line: 11, column: 71}}));
    }

    #[test]
    #[should_panic]
    fn bad_multiline() {
        let _tokens = "/* Bad multiline comment without termination".tokens().unwrap();
    }

}
