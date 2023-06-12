use crate::{token::{TokenMetadata, Token}, errors::ParserError, ast::{Expression, BinaryExpression, UnaryExpression, LiteralExpression, GroupingExpression}};

/*
GRAMMAR

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

pub struct Parser {
    pub tokens: Vec<TokenMetadata>,
    pub current: usize,
}

impl Parser {
    /// Create new parser with a list of tokens
    pub fn new(tokens: Vec<TokenMetadata>) -> Self {
        Parser { 
            tokens, 
            current: 0 
        }
    }

    /// Check is parser is at end of file
    fn is_at_end(&self) -> bool {
        self.peek().is_some_and(|t| t.token == Token::Eof)
    }

    /// Check if parsers current position is on a specific token
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is_some_and(|t| t.token == *token_type)
        }
    }

    /// Check if token at current position matches a set of token types
    /// If so, advance pointer
    fn matches(&mut self, types: &[Token]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Advance token pointer one step and return previous
    fn advance(&mut self) -> Option<&TokenMetadata> {
        self.current += 1;
        self.previous()
    }

    /// Peek at the character on the current pointer position
    fn peek(&self) -> Option<&TokenMetadata> {
        self.tokens.get(self.current)
    }

    /// Get the token at the previous pointer position
    fn previous(&self) -> Option<&TokenMetadata> {
        if self.current == 0 {
            None
        } else {
            self.tokens.get(self.current-1)
        }
    }

    /// Consume a specific token at the current position and move forward one step
    fn consume(&mut self, token: &Token) -> Result<(), ParserError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::Consume)
        }
    }

    /// Synchronize to next statement
    // fn synchronize(&mut self) {
    //     self.advance();

    //     while !self.is_at_end() {
    //         match self.peek().unwrap().token {
    //             Token::Semicolon => break,
    //             Token::Class => break,
    //             Token::Fn => break,
    //             Token::Let => break,
    //             Token::For => break,
    //             Token::If => break,
    //             Token::While => break,
    //             Token::Print => break,
    //             Token::Return => break,
    //             _ => {}
    //         }

    //         self.advance();
    //     }
    // }

    /// Parse the next expression
    pub fn parse(&mut self) -> Result<Expression,ParserError> {
        self.expression()
    }

    /// get an expression on the current pointer
    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.equality()
    }

    /// Try to parse an equality statement on the current position
    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.comparison()?;

        while self.matches(&[Token::BangEqual, Token::EqualEqual]) {
            let operator = self.previous().unwrap().token.clone();
            let right = Box::new(self.comparison()?);
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator,
                right,
            })
        }

        Ok(expression)
    }

    /// Try to parse a comparison on the current position of the pointer
    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.term()?;

        while self.matches(&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual]) {
            let operator = self.previous().unwrap().token.clone();
            let right = Box::new(self.term()?);
            expression = Expression::Binary(BinaryExpression { 
                left: Box::new(expression),
                operator, 
                right, 
            });
        }

        Ok(expression)
    }

    /// Try to parse terms on the current position of the pointer
    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.factor()?;

        while self.matches(&[Token::Minus, Token::Plus]) {
            let operator = self.previous().unwrap().token.clone();
            let right = Box::new(self.factor()?);
            expression = Expression::Binary(BinaryExpression { 
                left: Box::new(expression), 
                operator, 
                right 
            });
        }

        Ok(expression)
    }

    /// Try to parse factor on the current position of the pointer
    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.unary()?;

        while self.matches(&[Token::Slash, Token::Star]) {
            let operator = self.previous().unwrap().token.clone();
            let right = Box::new(self.unary()?);
            expression = Expression::Binary(BinaryExpression { 
                left: Box::new(expression), 
                operator, 
                right 
            });
        }

        Ok(expression)
    }

    /// Try to parse unary on the current position of the pointer
    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.matches(&[Token::Bang, Token::Minus]) {
            let operator = self.previous().unwrap().token.clone();
            let right = Box::new(self.unary()?);
            return Ok(Expression::Unary(UnaryExpression {
                operator,
                right
            }));
        }

        self.primary()
    }

    /// Try to parse a primary expression on the current position of the pointer
    fn primary(&mut self) -> Result<Expression, ParserError> {
        if self.matches(&[Token::False]) {
            return Ok(Expression::Literal(LiteralExpression{ literal: Token::False}));
        }
        if self.matches(&[Token::True]) {
            return Ok(Expression::Literal(LiteralExpression{ literal: Token::True}));
        }
        if self.matches(&[Token::Null]) {
            return Ok(Expression::Literal(LiteralExpression{ literal: Token::Null}));
        }

        match self.peek().unwrap().token.clone() {
            Token::String(s) => {
                self.advance();
                return Ok(Expression::Literal(LiteralExpression { literal: Token::String(s) }))
            },
            Token::Number(n) => {
                self.advance();
                return Ok(Expression::Literal(LiteralExpression { literal: Token::Number(n) }))
            },
            _ => {}
        }

        if self.matches(&[Token::LeftParenthesis]) {
            let expression = self.expression()?;
            self.consume(&Token::RightParenthesis)?;
            return Ok(Expression::Grouping(GroupingExpression {
                group: Box::new(expression)
            }))
        }


        Err(ParserError::Unexpected)
    }
}


#[cfg(test)]
pub mod tests {
    use crate::{scanner::Scannable, ast::AstPrinter};
    use super::*;


    #[test]
    fn basic() {
        let tokens =  "2*(4-1.123)".tokens().unwrap();
        
        let mut parser = Parser::new(tokens);
        let exp = parser.expression().unwrap();


        let printer = AstPrinter;
        let exp_str = printer.print(exp).unwrap();

        assert_eq!(exp_str, "(* 2 (group (- 4 1.123)))");
    }
}