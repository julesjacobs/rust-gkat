use crate::kernel1;
use crate::kernel2;
use crate::parsing::parse;
use crate::parsing::parse_user_friendly;
use crate::parsing::parse_expression;
use crate::syntax::gkat::Gkat;
use crate::syntax::PureBDDGkat;
use crate::Kernel;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(input: &str, kernel: Kernel) -> bool {
        let (exp1, exp2, expected_result) = parse(input.to_string());

        let mut gkat = PureBDDGkat::new();

        let result = match kernel {
            Kernel::k1 => {
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                solver.equiv_iter(&mut gkat, &exp1, &exp2)
            },
            Kernel::k2 => {
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                solver.equiv_iter(&mut gkat, i, j, &m, &n)
            },
        };

        assert_eq!(result, expected_result,
            "Equivalence check failed. Expected: {}, Got: {}",
            expected_result, result);

        result
    }

    // Function to test user-friendly syntax parsing and equivalence checking
    fn run_user_friendly_test(input: &str, kernel: Kernel) -> Result<bool, String> {
        // Parse the user-friendly input
        let parse_result = parse_user_friendly(input)?;
        let (exp1, exp2, _) = parse_result;

        println!("Parsed expressions:");
        println!("  exp1: {:?}", exp1);
        println!("  exp2: {:?}", exp2);

        let mut gkat = PureBDDGkat::new();

        let result = match kernel {
            Kernel::k1 => {
                let mut solver = kernel1::Solver::new();
                let exp1_bdd = gkat.from_exp(exp1);
                let exp2_bdd = gkat.from_exp(exp2);
                println!("Using k1 (Symbolic derivative)");
                solver.equiv_iter(&mut gkat, &exp1_bdd, &exp2_bdd)
            },
            Kernel::k2 => {
                let mut solver = kernel2::Solver::new();
                let exp1_bdd = gkat.from_exp(exp1);
                let exp2_bdd = gkat.from_exp(exp2);
                println!("Using k2 (Thompson's construction)");
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1_bdd);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2_bdd);
                solver.equiv_iter(&mut gkat, i, j, &m, &n)
            },
        };

        println!("Equivalence result: {}", result);
        Ok(result)
    }

    // Function to debug the user-friendly parser
    fn debug_user_friendly_parser(input: &str) {
        println!("Debugging parser for input: {}", input);

        // Try to parse with the user-friendly parser
        match parse_user_friendly(input) {
            Ok((exp1, exp2, expected)) => {
                println!("Successfully parsed:");
                println!("  exp1: {:?}", exp1);
                println!("  exp2: {:?}", exp2);
                println!("  expected: {}", expected);
            },
            Err(e) => {
                println!("Error parsing: {}", e);

                // Try to parse each side separately to identify the issue
                let parts: Vec<&str> = input.split("==").collect();
                if parts.len() == 2 {
                    let left = parts[0].trim();
                    let right = parts[1].trim();

                    println!("Trying to parse left side: {}", left);
                    match parse_expression(left) {
                        Ok(exp) => println!("  Left side parsed successfully: {:?}", exp),
                        Err(e) => println!("  Error parsing left side: {}", e),
                    }

                    println!("Trying to parse right side: {}", right);
                    match parse_expression(right) {
                        Ok(exp) => println!("  Right side parsed successfully: {:?}", exp),
                        Err(e) => println!("  Error parsing right side: {}", e),
                    }
                }
            }
        }
    }

    #[test]
    fn test_simple_k1() {
        let input = "p1 p1 (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_simple_k2() {
        let input = "p1 p1 (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }

    #[test]
    fn test_seq_k1() {
        let input = "(seq p1 p2) (seq p1 p2) (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_seq_k2() {
        let input = "(seq p1 p2) (seq p1 p2) (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }

    #[test]
    fn test_while_k1() {
        let input = "(while b1 (seq p2 p3)) (while b1 (seq p2 p3)) (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_while_k2() {
        let input = "(while b1 (seq p2 p3)) (while b1 (seq p2 p3)) (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }

    #[test]
    fn test_if_then_else_k1() {
        let input = "(if b1 p2 p3) (if b1 p2 p3) (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_if_then_else_k2() {
        let input = "(if b1 p2 p3) (if b1 p2 p3) (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }

    #[test]
    fn test_associativity_k1() {
        let input = "(seq p1 (seq p2 p3)) (seq (seq p1 p2) p3) (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_associativity_k2() {
        let input = "(seq p1 (seq p2 p3)) (seq (seq p1 p2) p3) (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }

    #[test]
    fn test_debug_user_friendly_if() {
        debug_user_friendly_parser("p1 ; (if b1 then p2 else p3) == (if b1 then p1 ; p2 else p1 ; p3)");
    }
}