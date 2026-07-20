//! Parser implementation for the Code programming language.

use lexer::token::{Token, TokenKind};

use crate::{
    ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator},
    error::ParserError,
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creates a new parser.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parses the token stream into an AST.
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current().kind {
            TokenKind::Const => self.parse_constant_declaration(),

            TokenKind::Func => self.parse_function_declaration(),

            TokenKind::Return => self.parse_return_statement(),

            TokenKind::If => self.parse_if_statement(),

            TokenKind::While => self.parse_while_statement(),

            TokenKind::For => self.parse_for_statement(),

            TokenKind::Break => self.parse_break_statement(),

            TokenKind::Continue => self.parse_continue_statement(),

            TokenKind::Identifier => {
                let expression = self.parse_expression()?;

                if self.current().kind == TokenKind::Equal {
                    self.advance();

                    let value = self.parse_expression()?;

                    match expression {
                        Expression::Identifier { name, span } => {
                            return Ok(Statement::Assignment { name, value, span });
                        }

                        Expression::Index {
                            object,
                            index,
                            span,
                        } => {
                            return Ok(Statement::IndexAssignment {
                                object: *object,
                                index: *index,
                                value,
                                span,
                            });
                        }

                        Expression::Property {
                            object,
                            property,
                            span,
                        } => {
                            return Ok(Statement::PropertyAssignment {
                                object: *object,
                                property,
                                value,
                                span,
                            });
                        }

                        _ => {
                            return Err(ParserError::UnexpectedToken);
                        }
                    }
                }

                Ok(Statement::Expression(expression))
            }

            _ => {
                let expression = self.parse_expression()?;
                Ok(Statement::Expression(expression))
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume if

        let condition = self.parse_expression()?;

        self.consume(TokenKind::LeftBrace)?;

        let mut then_branch = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            then_branch.push(self.parse_statement()?);
        }

        self.consume(TokenKind::RightBrace)?;

        let else_branch = if self.current().kind == TokenKind::Else {
            self.advance(); // consume else

            if self.current().kind == TokenKind::If {
                // else if ...
                Some(vec![self.parse_if_statement()?])
            } else {
                self.consume(TokenKind::LeftBrace)?;

                let mut statements = Vec::new();

                while self.current().kind != TokenKind::RightBrace {
                    statements.push(self.parse_statement()?);
                }

                self.consume(TokenKind::RightBrace)?;

                Some(statements)
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            span,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume while

        let condition = self.parse_expression()?;

        self.consume(TokenKind::LeftBrace)?;

        let mut body = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_statement()?);
        }

        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::While {
            condition,
            body,
            span,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume for

        let variable = self.consume(TokenKind::Identifier)?.lexeme.clone();

        self.consume(TokenKind::In)?;

        let iterable = self.parse_expression()?;

        self.consume(TokenKind::LeftBrace)?;

        let mut body = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_statement()?);
        }

        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::For {
            variable,
            iterable,
            body,
            span,
        })
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();
        // consume 'func'
        self.advance();

        // function name
        let name = self.consume(TokenKind::Identifier)?.lexeme.clone();

        // (
        self.consume(TokenKind::LeftParen)?;

        let mut parameters = Vec::new();

        if self.current().kind != TokenKind::RightParen {
            loop {
                let parameter = self.consume(TokenKind::Identifier)?.lexeme.clone();

                parameters.push(parameter);

                if self.current().kind != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        self.consume(TokenKind::RightParen)?;
        // {
        self.consume(TokenKind::LeftBrace)?;

        let mut body = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_statement()?);
        }

        // }
        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            body,
            span,
        })
    }

    fn parse_break_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume break

        Ok(Statement::Break { span })
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span;

        self.advance(); // consume continue

        Ok(Statement::Continue { span })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume return

        let value = self.parse_expression()?;

        Ok(Statement::Return { value, span })
    }

    fn parse_constant_declaration(&mut self) -> Result<Statement, ParserError> {
        let span = self.current().span.clone();

        self.advance(); // consume const

        let name = self.consume(TokenKind::Identifier)?.lexeme.clone();

        self.consume(TokenKind::Equal)?;

        let value = self.parse_expression()?;

        Ok(Statement::ConstantDeclaration { name, value, span })
    }

    // fn parse_assignment(&mut self) -> Result<Statement, ParserError> {
    //     let span = self.current().span.clone();

    //     let name = self.consume(TokenKind::Identifier)?.lexeme.clone();

    //     self.consume(TokenKind::Equal)?;

    //     let value = self.parse_expression()?;

    //     Ok(Statement::Assignment { name, value, span })
    // }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_or()
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        let operator = match self.current().kind {
            TokenKind::Plus => Some(UnaryOperator::Plus),
            TokenKind::Minus => Some(UnaryOperator::Minus),
            TokenKind::Not => Some(UnaryOperator::Not),
            _ => None,
        };

        if let Some(operator) = operator {
            let token = self.advance().clone();

            let expression = self.parse_unary()?;

            return Ok(Expression::Unary {
                operator,
                expression: Box::new(expression),
                span: token.span,
            });
        }

        self.parse_call()
    }
    fn parse_call(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary()?;

        loop {
            if self.current().kind == TokenKind::LeftParen {
                self.advance();

                let mut arguments = Vec::new();

                if self.current().kind != TokenKind::RightParen {
                    loop {
                        arguments.push(self.parse_expression()?);

                        if self.current().kind != TokenKind::Comma {
                            break;
                        }

                        self.advance();
                    }
                }

                self.consume(TokenKind::RightParen)?;

                let span = expression.span().clone();

                expression = Expression::Call {
                    callee: Box::new(expression),
                    arguments,
                    span,
                };
            } else if self.current().kind == TokenKind::LeftBracket {
                self.advance();

                let index = self.parse_expression()?;

                self.consume(TokenKind::RightBracket)?;

                let span = expression.span().clone();

                expression = Expression::Index {
                    object: Box::new(expression),
                    index: Box::new(index),
                    span,
                };
            } else if self.current().kind == TokenKind::Dot {
                self.advance();

                let property = self.consume_identifier()?;

                let span = expression.span().clone();

                expression = Expression::Property {
                    object: Box::new(expression),
                    property,
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_factor()?;

        loop {
            let operator = match self.current().kind {
                TokenKind::Plus => BinaryOperator::Plus,
                TokenKind::Minus => BinaryOperator::Minus,
                _ => {
                    break;
                }
            };

            self.advance();

            let right = self.parse_factor()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_term()?;

        loop {
            let operator = match self.current().kind {
                TokenKind::Less => BinaryOperator::Less,
                TokenKind::LessEqual => BinaryOperator::LessEqual,
                TokenKind::Greater => BinaryOperator::Greater,
                TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
                _ => {
                    break;
                }
            };

            self.advance();

            let right = self.parse_term()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_or(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_and()?;

        while self.current().kind == TokenKind::Or {
            self.advance();

            let right = self.parse_and()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_and(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_equality()?;

        while self.current().kind == TokenKind::And {
            self.advance();

            let right = self.parse_equality()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: BinaryOperator::And,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_comparison()?;

        loop {
            let operator = match self.current().kind {
                TokenKind::EqualEqual => BinaryOperator::EqualEqual,
                TokenKind::BangEqual => BinaryOperator::BangEqual,
                _ => {
                    break;
                }
            };

            self.advance();

            let right = self.parse_comparison()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_unary()?;

        loop {
            let operator = match self.current().kind {
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Percent => BinaryOperator::Modulo,
                _ => {
                    break;
                }
            };

            self.advance();

            let right = self.parse_unary()?;

            let span = expression.span().clone();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            };
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.current().kind {
            TokenKind::Number => {
                let token = self.advance().clone();

                Ok(Expression::NumberLiteral {
                    value: token.lexeme,
                    span: token.span,
                })
            }

            TokenKind::String => {
                let token = self.advance().clone();

                Ok(Expression::StringLiteral {
                    value: token.lexeme,
                    span: token.span,
                })
            }

            TokenKind::Identifier => {
                let token = self.advance().clone();

                Ok(Expression::Identifier {
                    name: token.lexeme,
                    span: token.span,
                })
            }

            TokenKind::True => {
                let token = self.advance().clone();

                Ok(Expression::BooleanLiteral {
                    value: true,
                    span: token.span,
                })
            }

            TokenKind::False => {
                let token = self.advance().clone();

                Ok(Expression::BooleanLiteral {
                    value: false,
                    span: token.span,
                })
            }

            TokenKind::Null => {
                let token = self.advance().clone();

                Ok(Expression::NullLiteral { span: token.span })
            }

            TokenKind::LeftParen => {
                self.advance();

                let expression = self.parse_expression()?;

                self.consume(TokenKind::RightParen)?;

                Ok(expression)
            }

            TokenKind::LeftBracket => {
                let span = self.advance().span;

                let mut elements = Vec::new();

                if self.current().kind != TokenKind::RightBracket {
                    loop {
                        elements.push(self.parse_expression()?);

                        if self.current().kind != TokenKind::Comma {
                            break;
                        }

                        self.advance();
                    }
                }

                self.consume(TokenKind::RightBracket)?;

                Ok(Expression::ArrayLiteral { elements, span })
            }

            TokenKind::LeftBrace => {
                let span = self.advance().span.clone();

                let mut properties = Vec::new();

                while !self.check(TokenKind::RightBrace) {
                    let key = self.consume_identifier()?;

                    self.consume(TokenKind::Colon)?;

                    let value = self.parse_expression()?;

                    properties.push((key, value));

                    if self.current().kind == TokenKind::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }

                self.consume(TokenKind::RightBrace)?;

                Ok(Expression::ObjectLiteral { properties, span })
            }

            _ => Err(ParserError::UnexpectedToken),
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }

    fn consume(&mut self, expected: TokenKind) -> Result<&Token, ParserError> {
        if self.current().kind == expected {
            Ok(self.advance())
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    fn consume_identifier(&mut self) -> Result<String, ParserError> {
        if self.current().kind == TokenKind::Identifier {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::EOF
    }
}
