use crate::token::Token;


pub enum ParserError {

}


pub trait ExpressionVisitor<T> {
    fn visit_unary(&self, expr: &UnaryExpression) -> Result<T, ParserError>;
    fn visit_binary(&self, expr: &BinaryExpression) -> Result<T, ParserError>;
    fn visit_grouping(&self, expr: &GroupingExpression) -> Result<T, ParserError>;
    fn visit_literal(&self, expr: &LiteralExpression) -> Result<T, ParserError>;
}

pub trait VisitorAccepter<T> {
    fn accept(&self, visitor: dyn ExpressionVisitor<T>) -> Result<T, ParserError>;
}

pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
}


pub struct UnaryExpression {
    operator: Token,
    right: Box<Expression>,
}

impl UnaryExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_unary(self)
    }
}


pub struct BinaryExpression {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

impl BinaryExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_binary(self)
    }
}

pub struct GroupingExpression {
    group: Box<Expression>,
}

impl GroupingExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_grouping(self)
    }
}


pub struct LiteralExpression {
    literal: Token,
}

impl LiteralExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_literal(self)
    }
}

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expression: Expression) -> Result<String, ParserError> {
        expression.accept(self)
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_unary(&self, expr: &UnaryExpression) -> Result<String, ParserError> {
        
    }

    fn visit_binary(&self, expr: &BinaryExpression) -> Result<String, ParserError> {
        
    }

    fn visit_grouping(&self, expr: &GroupingExpression) -> Result<String, ParserError> {
        
    }

    fn visit_literal(&self, expr: &LiteralExpression) -> Result<String, ParserError> {
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial() {
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Unary(UnaryExpression {
                operator: Token::Minus,
                right: Box::new(Expression::Literal(LiteralExpression { 
                    literal: Token::Number(123f64) 
                })),
            })),
            operator: Token::Star,
            right: Box::new(Expression::Grouping(GroupingExpression { 
                group: Box::new(Expression::Literal(LiteralExpression { 
                    literal: Token::Number(45.67f64),
                })),
            })),
        });

        println!("expr: {:#?}", expr);
    }
}