mod kernel1;
mod kernel2;
mod parsing;
pub mod syntax;
#[cfg(test)]
mod tests;

use parsing::*;
use syntax::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Kernel {
    k1, // Symbolic derivative method
    k2, // Symbolic Thompson's construction
}

#[wasm_bindgen]
pub struct GkatWasm {
    gkat: PureBDDGkat,
}

#[wasm_bindgen]
impl GkatWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gkat: PureBDDGkat::new(),
        }
    }

    pub fn check_equivalence(&mut self, input: &str, kernel: Kernel) -> bool {
        // Try to parse with the user-friendly parser first
        let (exp1, exp2, _expected_result) = match parse_user_friendly(input) {
            Ok(result) => result,
            Err(_) => {
                // If that fails, fall back to the original parser
                match parse(input.to_string()) {
                    (exp1, exp2, expected) => (exp1, exp2, expected)
                }
            }
        };

        let result = match kernel {
            Kernel::k1 => {
                let mut solver = kernel1::Solver::new();
                let exp1 = self.gkat.from_exp(exp1);
                let exp2 = self.gkat.from_exp(exp2);
                solver.equiv_iter(&mut self.gkat, &exp1, &exp2)
            },
            Kernel::k2 => {
                let mut solver = kernel2::Solver::new();
                let exp1 = self.gkat.from_exp(exp1);
                let exp2 = self.gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut self.gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut self.gkat, &exp2);
                solver.equiv_iter(&mut self.gkat, i, j, &m, &n)
            },
        };

        result
    }
}