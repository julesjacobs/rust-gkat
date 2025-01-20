use super::*;
use gxhash::HashMap;
use hashconsing::{HConsign, HashConsign};
use logicng::{
    formulas::{EncodedFormula, FormulaFactory},
    solver::minisat::*,
};

impl BExp for EncodedFormula {}

// Gkat based on BDD.
pub struct SATGkat {
    name_map: HashMap<String, EncodedFormula>,
    exp_hcons: HConsign<Exp_<EncodedFormula>>,
    // formula manager and solver
    solver: MiniSat,
    man: FormulaFactory,
    // caching
    is_false_cache: HashMap<EncodedFormula, bool>,
}

impl SATGkat {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::default(),
            exp_hcons: HConsign::empty(),
            solver: MiniSat::new(),
            man: FormulaFactory::new(),
            is_false_cache: HashMap::default(),
        }
    }
}

impl Gkat<EncodedFormula> for SATGkat {
    fn mk_zero(&mut self) -> EncodedFormula {
        self.man.constant(false)
    }

    fn mk_one(&mut self) -> EncodedFormula {
        self.man.constant(true)
    }

    fn mk_var(&mut self, s: String) -> EncodedFormula {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let x = self.man.variable(s.as_str());
        self.name_map.insert(s, x.clone());
        return x;
    }

    fn mk_and(&mut self, b1: &EncodedFormula, b2: &EncodedFormula) -> EncodedFormula {
        self.man.and(&[*b1, *b2])
    }

    fn mk_or(&mut self, b1: &EncodedFormula, b2: &EncodedFormula) -> EncodedFormula {
        self.man.or(&[*b1, *b2])
    }

    fn mk_not(&mut self, b: &EncodedFormula) -> EncodedFormula {
        self.man.not(*b)
    }

    fn is_false(&mut self, b: &EncodedFormula) -> bool {
        if let Some(result) = self.is_false_cache.get(b) {
            return *result;
        }
        self.solver.add(*b, &self.man);
        let result = match self.solver.sat() {
            sat::Tristate::True => false,
            sat::Tristate::False => true,
            sat::Tristate::Undef => panic!("unknown"),
        };
        self.solver.reset();
        self.is_false_cache.insert(*b, result);
        return result;
    }

    fn is_equiv(&mut self, b1: &EncodedFormula, b2: &EncodedFormula) -> bool {
        let b = self.man.equivalence(*b1, *b2);
        let nb = self.mk_not(&b);
        self.is_false(&nb)
    }

    fn hashcons(&mut self, e: Exp_<EncodedFormula>) -> Exp<EncodedFormula> {
        self.exp_hcons.mk(e)
    }
}
