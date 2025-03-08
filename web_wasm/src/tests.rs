use crate::kernel1;
use crate::kernel2;
use crate::parsing::parse;
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
    fn test_distributivity_k1() {
        let input = "(seq p1 (seq p2 p3)) (seq (seq p1 p2) p3) (equiv 1)";
        assert!(run_test(input, Kernel::k1));
    }

    #[test]
    fn test_distributivity_k2() {
        let input = "(seq p1 (seq p2 p3)) (seq (seq p1 p2) p3) (equiv 1)";
        assert!(run_test(input, Kernel::k2));
    }
}