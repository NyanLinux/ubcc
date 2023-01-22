use ast::{IfStatement, Statement};
use lex::tokens::Token;

use crate::{Parser, Precedence};

impl Parser {
    pub(super) fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip 'if'

        if self.current_token == Token::LParen {
            self.next_token();
        } else {
            return Err(format!(
                "expected token '(' but got {:?}",
                self.current_token
            ));
        }

        let condition = self.parse_expression(Precedence::Lowest)?;

        if self.peeked_token == Token::RParen {
            self.next_token(); // skip current
            self.next_token(); // skip ')'
        } else {
            return Err(format!(
                "expected token ')' but got {:?}",
                self.peeked_token
            ));
        }

        let consequence = self.parse_statement()?;

        let alternative = if self.peeked_token == Token::Else {
            self.next_token(); // skip current
            self.next_token(); // skip 'else'
            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(Statement::If(IfStatement::new(
            condition,
            consequence,
            alternative,
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::{
        BinaryExpression, BinaryOperator, Expression, IfStatement, InitDeclaration, Statement,
        Type, TypeEnum,
    };
    use lex::Lexer;

    #[test]
    fn test_parse_if_statement() {
        let cases = vec![
            (
                String::from("int a = 0; if (a == 0) return 0; "),
                vec![
                    Statement::InitDeclaration(InitDeclaration::new(
                        String::from("a"),
                        8,
                        Type::Primitive(TypeEnum::Int),
                        Some(Expression::Integer(0)),
                    )),
                    Statement::If(IfStatement::new(
                        Expression::Binary(BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("a"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Eq,
                            Expression::Integer(0),
                        )),
                        Statement::Return(Expression::Integer(0)),
                        None,
                    )),
                ],
            ),
            (
                String::from("int a = 0; if (a == 0) return 0; else return 1;"),
                vec![
                    Statement::InitDeclaration(InitDeclaration::new(
                        String::from("a"),
                        8,
                        Type::Primitive(TypeEnum::Int),
                        Some(Expression::Integer(0)),
                    )),
                    Statement::If(IfStatement::new(
                        Expression::Binary(BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("a"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Eq,
                            Expression::Integer(0),
                        )),
                        Statement::Return(Expression::Integer(0)),
                        Some(Statement::Return(Expression::Integer(1))),
                    )),
                ],
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }
}
