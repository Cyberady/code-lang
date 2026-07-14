//! Parser implementation for the Code programming language.

use lexer::token::{Token, TokenKind};

use crate::{
    ast::{BinaryOperator, Expression, Program, Statement},
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
            TokenKind::Const => self.parse_variable_declaration(),

            _ => {
                let expression = self.parse_expression()?;
                Ok(Statement::Expression(expression))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParserError> {
        self.advance(); // consume 'const'

        let name = self.consume(TokenKind::Identifier)?.lexeme.clone();

        self.consume(TokenKind::Equal)?;

        let value = self.parse_expression()?;

        Ok(Statement::VariableDeclaration { name, value })
    }
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_term()?;

        loop {
            if self.current().kind != TokenKind::LeftParen {
                break;
            }

            self.advance();

            let mut arguments = Vec::new();

            if self.current().kind != TokenKind::RightParen {
                arguments.push(self.parse_expression()?);
            }

            self.consume(TokenKind::RightParen)?;

            expression = Expression::Call {
                callee: Box::new(expression),
                arguments,
            };
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary()?;

        loop {
            let operator = match self.current().kind {
                TokenKind::Plus => BinaryOperator::Plus,
                TokenKind::Minus => BinaryOperator::Minus,
                _ => {
                    break;
                }
            };

            self.advance();

            let right = self.parse_primary()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.current().kind {
            TokenKind::Number => {
                let value = self.advance().lexeme.clone();
                Ok(Expression::NumberLiteral(value))
            }

            TokenKind::Identifier => {
                let value = self.advance().lexeme.clone();
                Ok(Expression::Identifier(value))
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

    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::EOF
    }
}
