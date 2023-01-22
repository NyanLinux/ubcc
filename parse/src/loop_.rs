use ast::{ForStatement, Statement, WhileStatement};
use lex::tokens::Token;

use crate::{Parser, Precedence};

impl Parser {
    pub(super) fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip 'while'

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

        let body = self.parse_statement()?;
        Ok(Statement::While(WhileStatement::new(condition, body)))
    }

    pub(super) fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip 'for'

        if self.current_token == Token::LParen {
            self.next_token();
        } else {
            return Err(format!(
                "expected token '(' but got {:?}",
                self.current_token
            ));
        }

        let init = if self.current_token == Token::SemiColon {
            None
        } else {
            Some(self.parse_expression_statement()?)
        };
        self.next_token(); // skip ';'

        let condition = if self.current_token == Token::SemiColon {
            None
        } else {
            let expr = self.parse_expression(Precedence::Lowest)?;
            self.next_token();
            if self.current_token == Token::SemiColon {
                self.next_token();
                Some(expr)
            } else {
                return Err(format!(
                    "expected token ';' but got {:?}",
                    self.current_token
                ));
            }
        };

        let step = if self.current_token == Token::RParen {
            None
        } else {
            let expr = self.parse_statement()?;
            if self.current_token == Token::RParen {
                self.next_token();
                Some(expr)
            } else {
                return Err(format!(
                    "expected token ')' but got {:?}",
                    self.current_token
                ));
            }
        };

        let body = self.parse_statement()?;

        Ok(Statement::For(ForStatement::new(
            init, condition, step, body,
        )))
    }
}

#[cfg(test)]
mod test {
    use ast::{BinaryExpression, BinaryOperator, Expression, InitDeclaration, Type, TypeEnum};
    use lex::Lexer;

    use super::*;

    #[test]
    fn test_parse_while_statement() {
        let cases = vec![(
            String::from("int a = 0; while (a == 0) return 0;"),
            vec![
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("a"),
                    8,
                    Type::Primitive(TypeEnum::Int),
                    Some(Expression::Integer(0)),
                )),
                Statement::While(WhileStatement::new(
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
                )),
            ],
        )];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }

    #[test]
    fn test_parse_for_statement() {
        let cases = vec![(
            String::from("int i = 0; for (i = 0; i < 10; i = i + 1) return 0;"),
            vec![
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("i"),
                    8,
                    Type::Primitive(TypeEnum::Int),
                    Some(Expression::Integer(0)),
                )),
                Statement::For(ForStatement::new(
                    Some(Statement::Expression(Expression::Binary(
                        BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("i"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Assignment,
                            Expression::Integer(0),
                        ),
                    ))),
                    Some(Expression::Binary(BinaryExpression::new(
                        Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        },
                        BinaryOperator::Lt,
                        Expression::Integer(10),
                    ))),
                    Some(Statement::Expression(Expression::Binary(
                        BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("i"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Assignment,
                            Expression::Binary(BinaryExpression::new(
                                Expression::LocalVariable {
                                    name: String::from("i"),
                                    offset: 8,
                                    type_: Type::Primitive(TypeEnum::Int),
                                },
                                BinaryOperator::Plus,
                                Expression::Integer(1),
                            )),
                        ),
                    ))),
                    Statement::Return(Expression::Integer(0)),
                )),
            ],
        )];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }
}
