use crate::token::Token;

#[derive(Debug)]
pub enum ParserError {  
    TokenIsNoUnary,
    TokenIsNoBinary,
    TokenIsNoLiteral,
}


pub trait ExpressionVisitor<T> {
    fn visit_unary(&self, expr: &UnaryExpression) -> Result<T, ParserError>;
    fn visit_binary(&self, expr: &BinaryExpression) -> Result<T, ParserError>;
    fn visit_grouping(&self, expr: &GroupingExpression) -> Result<T, ParserError>;
    fn visit_literal(&self, expr: &LiteralExpression) -> Result<T, ParserError>;
}

#[derive(Debug)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
}

impl Expression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        match self {
            Self::Unary(unary) => unary.accept(visitor),
            Self::Binary(binary) => binary.accept(visitor),
            Self::Grouping(grouping) => grouping.accept(visitor),
            Self::Literal(literal) => literal.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_unary(self)
    }
}


#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_binary(self)
    }
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub group: Box<Expression>,
}

impl GroupingExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_grouping(self)
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub literal: Token,
}

impl LiteralExpression {
    fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ParserError> {
        visitor.visit_literal(self)
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expression: Expression) -> Result<String, ParserError> {
        expression.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&Expression]) -> Result<String, ParserError> {
        let mut out = String::from('(');
        out.push_str(name);
        for expr in expressions {
            out.push(' ');
            out.push_str(&expr.accept(self)?)
        }

        out.push(')');
        Ok(out)
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_unary(&self, expr: &UnaryExpression) -> Result<String, ParserError> {
        let name = match &expr.operator {
            Token::Minus => "-",
            _ => return Err(ParserError::TokenIsNoUnary),
        };

        self.parenthesize(name, &[&expr.right])
    }

    fn visit_binary(&self, expr: &BinaryExpression) -> Result<String, ParserError> {
        let name = match &expr.operator {
            Token::Plus => "+",
            Token::Star => "*",
            _ => return Err(ParserError::TokenIsNoBinary),
        };

        self.parenthesize(name, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &GroupingExpression) -> Result<String, ParserError> {
        self.parenthesize("group", &[&expr.group])
    }

    fn visit_literal(&self, expr: &LiteralExpression) -> Result<String, ParserError> {
       match &expr.literal {
            Token::String(str) => Ok(String::from(str)),
            Token::Number(nbr) => Ok(nbr.to_string()),
            _ => Err(ParserError::TokenIsNoLiteral),
       }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
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
    
        let printer = AstPrinter;
        let exp = printer.print(expr).unwrap();

        assert_eq!(exp, "(* (- 123) (group 45.67))");
    }
}