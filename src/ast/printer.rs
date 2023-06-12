use super::types::*;
use crate::errors::{ScriptError};
use crate::token::Token;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expression: Expression) -> Result<String, ScriptError> {
        expression.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&Expression]) -> Result<String, ScriptError> {
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
    fn visit_unary(&self, expr: &UnaryExpression) -> Result<String, ScriptError> {
        let name = match &expr.operator {
            Token::Minus => "-",
            _ => return Err(ScriptError::AstPrinterError),
        };

        self.parenthesize(name, &[&expr.right])
    }

    fn visit_binary(&self, expr: &BinaryExpression) -> Result<String, ScriptError> {
        let name = match &expr.operator {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            _ => return Err(ScriptError::AstPrinterError),
        };

        self.parenthesize(name, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &GroupingExpression) -> Result<String, ScriptError> {
        self.parenthesize("group", &[&expr.group])
    }

    fn visit_literal(&self, expr: &LiteralExpression) -> Result<String, ScriptError> {
       match &expr.literal {
            Token::String(str) => Ok(String::from(str)),
            Token::Number(nbr) => Ok(nbr.to_string()),
            _ => Err(ScriptError::AstPrinterError),
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