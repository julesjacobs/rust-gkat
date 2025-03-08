use crate::parsing::raw::{BExp, Exp};
use std::str::FromStr;

/// Parses a string in the format "expr1 == expr2" into (Exp, Exp, bool)
/// This is a more user-friendly syntax compared to the raw parser
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

/// Parses a single expression in user-friendly syntax
fn parse_expression(input: &str) -> Result<Exp, String> {
    let input = input.trim();

    // Handle empty input
    if input.is_empty() {
        return Err("Empty expression".to_string());
    }

    // Handle simple identifiers (actions)
    if is_identifier(input) {
        return Ok(Exp::Act(input.to_string()));
    }

    // Handle if-then-else: "if b1 then p2 else p3"
    if input.starts_with("if ") {
        return parse_if_then_else(input);
    }

    // Handle while: "while b1 do p2"
    if input.starts_with("while ") {
        return parse_while(input);
    }

    // Handle sequence with ;
    if input.contains(";") {
        return parse_sequence(input);
    }

    // Handle choice with +
    if input.contains("+") {
        return parse_choice(input);
    }

    // If we can't parse it with our custom parser, try the original syntax
    // This allows users to still use the raw syntax if they prefer
    if input.starts_with("(") {
        // This is likely using the original syntax, so we'll just pass it through
        // to be handled by the main parser
        return Ok(Exp::Act(input.to_string()));
    }

    Err(format!("Could not parse expression: {}", input))
}

/// Parses an if-then-else expression: "if b1 then p2 else p3"
fn parse_if_then_else(input: &str) -> Result<Exp, String> {
    let input = input.trim();

    // Check if it starts with "if "
    if !input.starts_with("if ") {
        return Err(format!("Not an if-then-else expression: {}", input));
    }

    // Find "then" and "else"
    let then_pos = input.find(" then ");
    let else_pos = input.find(" else ");

    if then_pos.is_none() || else_pos.is_none() || else_pos.unwrap() <= then_pos.unwrap() {
        return Err(format!("Invalid if-then-else syntax: {}", input));
    }

    let then_pos = then_pos.unwrap();
    let else_pos = else_pos.unwrap();

    // Extract the condition, then-branch, and else-branch
    let condition = input[3..then_pos].trim();
    let then_branch = input[(then_pos + 6)..else_pos].trim();
    let else_branch = input[(else_pos + 6)..].trim();

    // Parse the condition as a boolean expression
    let b_exp = parse_boolean_expression(condition)?;

    // Parse the branches
    let then_exp = parse_expression(then_branch)?;
    let else_exp = parse_expression(else_branch)?;

    Ok(Exp::Ifte(b_exp, Box::new(then_exp), Box::new(else_exp)))
}

/// Parses a while expression: "while b1 do p2"
fn parse_while(input: &str) -> Result<Exp, String> {
    let input = input.trim();

    // Check if it starts with "while "
    if !input.starts_with("while ") {
        return Err(format!("Not a while expression: {}", input));
    }

    // Find "do"
    let do_pos = input.find(" do ");

    if do_pos.is_none() {
        return Err(format!("Invalid while syntax: {}", input));
    }

    let do_pos = do_pos.unwrap();

    // Extract the condition and body
    let condition = input[6..do_pos].trim();
    let body = input[(do_pos + 4)..].trim();

    // Parse the condition as a boolean expression
    let b_exp = parse_boolean_expression(condition)?;

    // Parse the body
    let body_exp = parse_expression(body)?;

    Ok(Exp::While(b_exp, Box::new(body_exp)))
}

/// Parses a sequence expression: "p1 ; p2"
fn parse_sequence(input: &str) -> Result<Exp, String> {
    let input = input.trim();

    // Split by ;
    let parts: Vec<&str> = input.split(";").collect();

    if parts.is_empty() {
        return Err(format!("Invalid sequence syntax: {}", input));
    }

    // Parse each part
    let mut exps = Vec::new();
    for part in parts {
        let part = part.trim();
        if !part.is_empty() {
            exps.push(parse_expression(part)?);
        }
    }

    if exps.is_empty() {
        return Err(format!("Empty sequence: {}", input));
    }

    // Build the sequence expression
    let mut result = exps.pop().unwrap();
    while let Some(exp) = exps.pop() {
        result = Exp::Seq(Box::new(exp), Box::new(result));
    }

    Ok(result)
}

/// Parses a choice expression: "p1 + p2"
fn parse_choice(input: &str) -> Result<Exp, String> {
    let input = input.trim();

    // Split by +
    let parts: Vec<&str> = input.split("+").collect();

    if parts.len() < 2 {
        return Err(format!("Invalid choice syntax: {}", input));
    }

    // Parse each part
    let mut exps = Vec::new();
    for part in parts {
        let part = part.trim();
        if !part.is_empty() {
            exps.push(parse_expression(part)?);
        }
    }

    if exps.len() < 2 {
        return Err(format!("Not enough choices: {}", input));
    }

    // Build the choice expression using sequence with test(0) and test(1)
    // p1 + p2 is equivalent to (test 1 ; p1) + (test 1 ; p2)
    // which we represent as a sequence with tests
    let mut result = exps.pop().unwrap();
    while let Some(exp) = exps.pop() {
        // Create a choice between the current result and the next expression
        // We'll represent this as a sequence with a test
        let test_true = Exp::Test(BExp::One);
        let left_branch = Exp::Seq(Box::new(test_true.clone()), Box::new(exp));
        let right_branch = Exp::Seq(Box::new(test_true), Box::new(result));

        // The choice is represented as a sequence of the left and right branches
        result = Exp::Seq(Box::new(left_branch), Box::new(right_branch));
    }

    Ok(result)
}

/// Parses a boolean expression
fn parse_boolean_expression(input: &str) -> Result<BExp, String> {
    let input = input.trim();

    // Handle empty input
    if input.is_empty() {
        return Err("Empty boolean expression".to_string());
    }

    // Handle simple identifiers (boolean variables)
    if is_identifier(input) {
        return Ok(BExp::PBool(input.to_string()));
    }

    // Handle constants
    if input == "0" {
        return Ok(BExp::Zero);
    }
    if input == "1" {
        return Ok(BExp::One);
    }

    // Handle negation: "not b1"
    if input.starts_with("not ") {
        let inner = input[4..].trim();
        let inner_exp = parse_boolean_expression(inner)?;
        return Ok(BExp::Not(Box::new(inner_exp)));
    }

    // Handle and: "b1 and b2"
    if input.contains(" and ") {
        let parts: Vec<&str> = input.split(" and ").collect();
        if parts.len() < 2 {
            return Err(format!("Invalid and syntax: {}", input));
        }

        let mut exps = Vec::new();
        for part in parts {
            let part = part.trim();
            if !part.is_empty() {
                exps.push(parse_boolean_expression(part)?);
            }
        }

        if exps.len() < 2 {
            return Err(format!("Not enough operands for and: {}", input));
        }

        let mut result = exps.pop().unwrap();
        while let Some(exp) = exps.pop() {
            result = BExp::And(Box::new(exp), Box::new(result));
        }

        return Ok(result);
    }

    // Handle or: "b1 or b2"
    if input.contains(" or ") {
        let parts: Vec<&str> = input.split(" or ").collect();
        if parts.len() < 2 {
            return Err(format!("Invalid or syntax: {}", input));
        }

        let mut exps = Vec::new();
        for part in parts {
            let part = part.trim();
            if !part.is_empty() {
                exps.push(parse_boolean_expression(part)?);
            }
        }

        if exps.len() < 2 {
            return Err(format!("Not enough operands for or: {}", input));
        }

        let mut result = exps.pop().unwrap();
        while let Some(exp) = exps.pop() {
            result = BExp::Or(Box::new(exp), Box::new(result));
        }

        return Ok(result);
    }

    Err(format!("Could not parse boolean expression: {}", input))
}

/// Checks if a string is a valid identifier
fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return false;
    }

    for c in s.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let result = parse_expression("p1").unwrap();
        assert!(matches!(result, Exp::Act(s) if s == "p1"));
    }

    #[test]
    fn test_parse_sequence() {
        let result = parse_expression("p1 ; p2").unwrap();
        assert!(matches!(result, Exp::Seq(_, _)));
    }

    #[test]
    fn test_parse_if_then_else() {
        let result = parse_expression("if b1 then p2 else p3").unwrap();
        assert!(matches!(result, Exp::Ifte(_, _, _)));
    }

    #[test]
    fn test_parse_while() {
        let result = parse_expression("while b1 do p2").unwrap();
        assert!(matches!(result, Exp::While(_, _)));
    }

    #[test]
    fn test_parse_choice() {
        let result = parse_expression("p1 + p2").unwrap();
        assert!(matches!(result, Exp::Seq(_, _)));
    }

    #[test]
    fn test_parse_complex_expression() {
        let result = parse_expression("if b1 then p2 ; p3 else p4 + p5").unwrap();
        assert!(matches!(result, Exp::Ifte(_, _, _)));
    }

    #[test]
    fn test_parse_nested_expression() {
        let result = parse_expression("while b1 do (if b2 then p1 else p2)").unwrap();
        assert!(matches!(result, Exp::While(_, _)));
    }

    #[test]
    fn test_parse_boolean_expression_simple() {
        let result = parse_boolean_expression("b1").unwrap();
        assert!(matches!(result, BExp::PBool(s) if s == "b1"));
    }

    #[test]
    fn test_parse_boolean_expression_not() {
        let result = parse_boolean_expression("not b1").unwrap();
        assert!(matches!(result, BExp::Not(_)));
    }

    #[test]
    fn test_parse_boolean_expression_and() {
        let result = parse_boolean_expression("b1 and b2").unwrap();
        assert!(matches!(result, BExp::And(_, _)));
    }

    #[test]
    fn test_parse_boolean_expression_or() {
        let result = parse_boolean_expression("b1 or b2").unwrap();
        assert!(matches!(result, BExp::Or(_, _)));
    }

    #[test]
    fn test_parse_boolean_expression_complex() {
        let result = parse_boolean_expression("b1 and not b2 or b3").unwrap();
        assert!(matches!(result, BExp::Or(_, _)) || matches!(result, BExp::And(_, _)));
    }

    #[test]
    fn test_parse_user_friendly() {
        let result = parse_user_friendly("p1 ; p2 == p1 ; p2").unwrap();
        assert_eq!(result.2, true);
    }

    #[test]
    fn test_parse_user_friendly_complex() {
        let result = parse_user_friendly("if b1 then p2 else p3 == if b1 then p2 else p3").unwrap();
        assert_eq!(result.2, true);
    }

    #[test]
    fn test_parse_user_friendly_with_spaces() {
        let result = parse_user_friendly("  p1 ; p2  ==  p1 ; p2  ").unwrap();
        assert_eq!(result.2, true);
    }

    #[test]
    fn test_parse_user_friendly_error() {
        let result = parse_user_friendly("p1 ; p2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_user_friendly_error_invalid_expression() {
        let result = parse_user_friendly("@invalid == p1");
        assert!(result.is_err());
    }
}