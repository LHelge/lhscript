use crate::token::Token;
use crate::errors::ScriptError;

macro_rules! define_ast_types {
    ($($enum:ident, $name:ident, $visit:ident { $($prop_name:ident: $prop_type:ty),* },)*) => {
        pub enum Expression {
            $(
                $enum($name),
            )*
        }

        impl Expression {
            pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ScriptError> {
                match self {
                    $(
                        Self::$enum(e) => e.accept(visitor),
                    )*
                }
            }
        }

        pub trait ExpressionVisitor<T> {
            $(
                fn $visit(&self, expression: &$name) -> Result<T, ScriptError>;
            )*
        }

        $(
            pub struct $name {
                $(
                    pub $prop_name: $prop_type,
                )*
            }

            impl $name {
                fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, ScriptError> {
                    visitor.$visit(self)
                }
            }
        )*
    }
}

define_ast_types!(
    Unary, UnaryExpression, visit_unary {operator: Token, right: Box<Expression>},
    Binary, BinaryExpression, visit_binary {left: Box<Expression>, operator: Token, right: Box<Expression>},
    Grouping, GroupingExpression, visit_grouping {group: Box<Expression>},
    Literal, LiteralExpression, visit_literal {literal: Token},
);