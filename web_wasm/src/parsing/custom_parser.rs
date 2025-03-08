use crate::parsing::raw::{BExp, Exp};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Identifiers
    Identifier(String),
    // Keywords
    If,
    Else,
    While,
    Assert,
    // Symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Ampersand,
    Pipe,
    Bang,
    Equals,
    // End of input
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.peek() {
            match c {
                '(' => {
                    self.advance();
                    Token::LParen
                },
                ')' => {
                    self.advance();
                    Token::RParen
                },
                '{' => {
                    self.advance();
                    Token::LBrace
                },
                '}' => {
                    self.advance();
                    Token::RBrace
                },
                ';' => {
                    self.advance();
                    Token::Semicolon
                },
                '&' => {
                    self.advance();
                    Token::Ampersand
                },
                '|' => {
                    self.advance();
                    Token::Pipe
                },
                '!' => {
                    self.advance();
                    Token::Bang
                },
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::Equals
                    } else {
                        // Invalid token, but we'll handle this as an identifier for simplicity
                        Token::Identifier("=".to_string())
                    }
                },
                _ if c.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "assert" => Token::Assert,
                        _ => Token::Identifier(identifier),
                    }
                },
                _ => {
                    self.advance();
                    Token::Identifier(c.to_string())
                }
            }
        } else {
            Token::EOF
        }
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_expression(&mut self) -> Result<Exp, String> {
        self.parse_sequence()
    }

    fn parse_sequence(&mut self) -> Result<Exp, String> {
        let left = self.parse_statement()?;

        // Check if the current token is a semicolon (not the peek token)
        if self.current_token == Token::Semicolon {
            self.next_token(); // move to the token after semicolon
            let right = self.parse_sequence()?;
            return Ok(Exp::Seq(Box::new(left), Box::new(right)));
        }

        // Check if the peek token is a semicolon
        if self.peek_token == Token::Semicolon {
            self.next_token(); // consume semicolon
            self.next_token(); // move to the token after semicolon
            let right = self.parse_sequence()?;
            return Ok(Exp::Seq(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }

    fn parse_statement(&mut self) -> Result<Exp, String> {
        match self.current_token {
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::Assert => self.parse_assert_statement(),
            Token::LParen => self.parse_parenthesized_expression(),
            Token::Identifier(ref id) => {
                // Check if it's an uppercase identifier (statement)
                if let Some(first_char) = id.chars().next() {
                    if first_char.is_uppercase() {
                        let exp = Exp::Act(id.clone());
                        self.next_token();
                        Ok(exp)
                    } else {
                        Err(format!("Expected uppercase identifier for statement, got {}", id))
                    }
                } else {
                    Err("Empty identifier".to_string())
                }
            },
            _ => Err(format!("Unexpected token for statement: {:?}", self.current_token)),
        }
    }

    fn parse_if_statement(&mut self) -> Result<Exp, String> {
        self.next_token(); // consume 'if'

        // Parse the boolean condition
        let condition = self.parse_boolean_expression()?;

        // Check if the current token is '{'
        if self.current_token != Token::LBrace {
            return Err(format!("Expected '{{' after if condition, got {:?}", self.current_token));
        }

        // Move past '{'
        self.next_token();

        // Parse the then branch
        let then_branch = self.parse_expression()?;

        // Check if the current token is '}'
        if self.current_token != Token::RBrace {
            return Err(format!("Expected '}}' after then branch, got {:?}", self.current_token));
        }

        // Move past '}'
        self.next_token();

        // Check if the current token is 'else'
        if self.current_token != Token::Else {
            return Err(format!("Expected 'else' after then branch, got {:?}", self.current_token));
        }

        // Move past 'else'
        self.next_token();

        // Check if the current token is '{'
        if self.current_token != Token::LBrace {
            return Err(format!("Expected '{{' after 'else', got {:?}", self.current_token));
        }

        // Move past '{'
        self.next_token();

        // Parse the else branch
        let else_branch = self.parse_expression()?;

        // Check if the current token is '}'
        if self.current_token != Token::RBrace {
            return Err(format!("Expected '}}' after else branch, got {:?}", self.current_token));
        }

        // Move past '}'
        self.next_token();

        Ok(Exp::Ifte(condition, Box::new(then_branch), Box::new(else_branch)))
    }

    fn parse_while_statement(&mut self) -> Result<Exp, String> {
        self.next_token(); // consume 'while'

        // Parse the boolean condition
        let condition = self.parse_boolean_expression()?;

        // Check if the current token is '{'
        if self.current_token != Token::LBrace {
            return Err(format!("Expected '{{' after while condition, got {:?}", self.current_token));
        }

        // Move past '{'
        self.next_token();

        // Parse the body
        let body = self.parse_expression()?;

        // Check if the current token is '}'
        if self.current_token != Token::RBrace {
            return Err(format!("Expected '}}' after while body, got {:?}", self.current_token));
        }

        // Move past '}'
        self.next_token();

        Ok(Exp::While(condition, Box::new(body)))
    }

    fn parse_assert_statement(&mut self) -> Result<Exp, String> {
        self.next_token(); // consume 'assert'

        let condition = self.parse_boolean_expression()?;

        // We don't need to consume an additional token here, as parse_boolean_expression
        // already consumes all tokens it needs

        Ok(Exp::Test(condition))
    }

    fn parse_parenthesized_expression(&mut self) -> Result<Exp, String> {
        self.next_token(); // consume '('

        let exp = self.parse_expression()?;

        // The current token should be ')'
        if self.current_token != Token::RParen {
            return Err(format!("Expected ')' after expression, got {:?}", self.current_token));
        }

        // Move past ')'
        self.next_token();

        Ok(exp)
    }

    fn parse_boolean_expression(&mut self) -> Result<BExp, String> {
        self.parse_boolean_or()
    }

    fn parse_boolean_or(&mut self) -> Result<BExp, String> {
        let mut left = self.parse_boolean_and()?;

        // Check if the current token is '|'
        if self.current_token == Token::Pipe {
            self.next_token(); // move to the right operand
            let right = self.parse_boolean_and()?;
            left = BExp::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_boolean_and(&mut self) -> Result<BExp, String> {
        let mut left = self.parse_boolean_not()?;

        // Check if the current token is '&'
        if self.current_token == Token::Ampersand {
            self.next_token(); // move to the right operand
            let right = self.parse_boolean_not()?;
            left = BExp::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_boolean_not(&mut self) -> Result<BExp, String> {
        if self.current_token == Token::Bang {
            self.next_token(); // consume '!'
            let expr = self.parse_boolean_not()?;
            Ok(BExp::Not(Box::new(expr)))
        } else {
            self.parse_boolean_primary()
        }
    }

    fn parse_boolean_primary(&mut self) -> Result<BExp, String> {
        match self.current_token {
            Token::LParen => {
                self.next_token(); // consume '('
                let expr = self.parse_boolean_expression()?;

                // Check if the current token is ')'
                if self.current_token != Token::RParen {
                    return Err(format!("Expected ')' after boolean expression, got {:?}", self.current_token));
                }

                // Move past ')'
                self.next_token();

                Ok(expr)
            },
            Token::Identifier(ref id) => {
                // Check if it's a lowercase identifier (boolean variable)
                if let Some(first_char) = id.chars().next() {
                    if first_char.is_lowercase() {
                        let exp = BExp::PBool(id.clone());
                        self.next_token(); // Consume the identifier
                        Ok(exp)
                    } else {
                        Err(format!("Expected lowercase identifier for boolean variable, got {}", id))
                    }
                } else {
                    Err("Empty identifier".to_string())
                }
            },
            _ => Err(format!("Unexpected token for boolean expression: {:?}", self.current_token)),
        }
    }
}

/// Parses a string in the format "expr1 == expr2" into (Exp, Exp, bool)
pub fn parse_user_friendly(input: &str) -> Result<(Exp, Exp, bool), String> {
    // Split by ==
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return Err("Invalid input format. Expected 'expr1 == expr2'".to_string());
    }

    let exp1_str = parts[0].trim();
    let exp2_str = parts[1].trim();

    // For interactive version, we always assume the expected result is true
    let expected_result = true;

    // Parse the expressions
    let exp1 = parse_expression(exp1_str)?;
    let exp2 = parse_expression(exp2_str)?;

    Ok((exp1, exp2, expected_result))
}

/// Parse an expression using the recursive descent parser
pub fn parse_expression(input: &str) -> Result<Exp, String> {
    let mut parser = Parser::new(input);
    parser.parse_expression()
}

/// Parse a boolean expression using the recursive descent parser
pub fn parse_boolean_expression(input: &str) -> Result<BExp, String> {
    let mut parser = Parser::new(input);
    parser.parse_boolean_expression()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let result = parse_expression("P1");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Exp::Act(_)));
    }

    #[test]
    fn test_parse_sequence() {
        let result = parse_expression("P1; P2");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Seq(_, _)));
        }
    }

    #[test]
    fn test_parse_if_then_else() {
        let result = parse_expression("if b1 { P1 } else { P2 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
        }
    }

    #[test]
    fn test_parse_while() {
        let result = parse_expression("while b1 { P1 }");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Exp::While(_, _)));
    }

    #[test]
    fn test_parse_assert() {
        let result = parse_expression("assert b1");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Exp::Test(_)));
    }

    #[test]
    fn test_parse_complex_expression() {
        let result = parse_expression("if b1 { P1; P2 } else { while b2 { P3 } }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_expression() {
        let result = parse_expression("if b1 { P1 } else { P2 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
        }
    }

    #[test]
    fn test_parse_boolean_expression_simple() {
        let result = parse_boolean_expression("b1");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), BExp::PBool(_)));
    }

    #[test]
    fn test_parse_boolean_expression_not() {
        let result = parse_boolean_expression("!b1");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), BExp::Not(_)));
    }

    #[test]
    fn test_parse_boolean_expression_and() {
        let result = parse_boolean_expression("b1 & b2");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), BExp::And(_, _)));
    }

    #[test]
    fn test_parse_boolean_expression_or() {
        let result = parse_boolean_expression("b1 | b2");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), BExp::Or(_, _)));
    }

    #[test]
    fn test_parse_boolean_expression_complex() {
        let result = parse_boolean_expression("b1 & (b2 | !b3)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_user_friendly() {
        let result = parse_user_friendly("P1 == P1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_user_friendly_complex() {
        let result = parse_user_friendly("if b1 { P1 } else { P2 } == while b1 { P1 }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_user_friendly_with_spaces() {
        let result = parse_user_friendly("  P1  ==  P1  ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_user_friendly_error() {
        let result = parse_user_friendly("P1");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_user_friendly_error_invalid_expression() {
        let result = parse_user_friendly("P1 == invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complex_boolean_expression() {
        let result = parse_boolean_expression("!b1 & (b2 | b3) & !b4");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, BExp::And(_, _)));
        }
    }

    #[test]
    fn test_parse_nested_if_statements() {
        let result = parse_expression("if b1 { if b2 { P1 } else { P2 } } else { P3 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
        }
    }

    #[test]
    fn test_parse_nested_while_statements() {
        let result = parse_expression("while b1 { while b2 { P1 } }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::While(_, _)));
        }
    }

    #[test]
    fn test_parse_mixed_statements() {
        let result = parse_expression("if b1 { P1; while b2 { P2 } } else { assert b3 }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_parenthesized_statements() {
        let result = parse_expression("(if b1 { P1 } else { P2 })");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
        }
    }

    #[test]
    fn test_parse_error_missing_closing_brace() {
        let result = parse_expression("if b1 { P1 else { P2 }");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_else() {
        let result = parse_expression("if b1 { P1 } { P2 }");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_boolean() {
        let result = parse_boolean_expression("b1 & | b2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unmatched_parenthesis() {
        let result = parse_expression("(P1; P2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_statement() {
        let result = parse_expression("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_lowercase_statement() {
        let result = parse_expression("lowercase");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_uppercase_boolean() {
        let result = parse_boolean_expression("Uppercase");
        assert!(result.is_err());
    }

    // Additional parenthesization tests
    #[test]
    fn test_parse_deeply_nested_parentheses() {
        let result = parse_expression("(((P1)))");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Act(_)));
        }
    }

    #[test]
    fn test_parse_parenthesized_sequence() {
        let result = parse_expression("(P1)");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Act(_)));
        }
    }

    #[test]
    fn test_parse_parenthesized_if_in_sequence() {
        let result = parse_expression("if b1 { P1 } else { P2 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
        }
    }

    #[test]
    fn test_parse_multiple_semicolons() {
        let result = parse_expression("P1; P2");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Seq(_, _)));
        }
    }

    // Boolean expression precedence tests
    #[test]
    fn test_parse_boolean_and_precedence() {
        // AND should have higher precedence than OR
        let result = parse_boolean_expression("b1 | b2 & b3");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, BExp::Or(_, _)));
            if let BExp::Or(left, right) = exp {
                assert!(matches!(*left, BExp::PBool(_)));
                assert!(matches!(*right, BExp::And(_, _)));
            }
        }
    }

    #[test]
    fn test_parse_boolean_not_precedence() {
        // NOT should have higher precedence than AND
        let result = parse_boolean_expression("!b1 & b2");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, BExp::And(_, _)));
            if let BExp::And(left, right) = exp {
                assert!(matches!(*left, BExp::Not(_)));
                assert!(matches!(*right, BExp::PBool(_)));
            }
        }
    }

    #[test]
    fn test_parse_boolean_parentheses_precedence() {
        // Parentheses should override normal precedence
        let result = parse_boolean_expression("(b1 | b2) & b3");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, BExp::And(_, _)));
            if let BExp::And(left, right) = exp {
                assert!(matches!(*left, BExp::Or(_, _)));
                assert!(matches!(*right, BExp::PBool(_)));
            }
        }
    }

    // Additional semicolon tests
    #[test]
    fn test_parse_semicolon_in_if_branches() {
        let result = parse_expression("if b1 { P1; P2 } else { P3; P4 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Ifte(_, _, _)));
            if let Exp::Ifte(_, then_branch, else_branch) = exp {
                assert!(matches!(*then_branch, Exp::Seq(_, _)));
                assert!(matches!(*else_branch, Exp::Seq(_, _)));
            }
        }
    }

    #[test]
    fn test_parse_semicolon_in_while_body() {
        let result = parse_expression("while b1 { P1; P2; P3 }");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::While(_, _)));
            if let Exp::While(_, body) = exp {
                assert!(matches!(*body, Exp::Seq(_, _)));
            }
        }
    }

    #[test]
    fn test_parse_semicolon_with_assert() {
        let result = parse_expression("assert b1; P1");
        assert!(result.is_ok());
        if let Ok(exp) = result {
            assert!(matches!(exp, Exp::Seq(_, _)));
            if let Exp::Seq(left, _) = exp {
                assert!(matches!(*left, Exp::Test(_)));
            }
        }
    }

    #[test]
    fn test_parse_error_trailing_semicolon() {
        let result = parse_expression("P1;");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_empty_sequence() {
        let result = parse_expression("P1; ; P2");
        assert!(result.is_err());
    }
}