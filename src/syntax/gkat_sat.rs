use super::*;
use gxhash::{GxBuildHasher, HashMap};
use hashconsing::{HConsign, HashConsign};
use logicng::{
    formulas::{EncodedFormula, FormulaFactory},
    solver::minisat::*,
};

impl BExp for EncodedFormula {}

// Gkat based on BDD.
pub struct SATGkat {
    name_map: HashMap<String, EncodedFormula>,
    exp_hcons: HConsign<Exp_<EncodedFormula>, GxBuildHasher>,
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
            exp_hcons: HConsign::with_hasher(GxBuildHasher::default()),
            solver: MiniSat::new(),
            man: FormulaFactory::new(),
            is_false_cache: HashMap::default(),
        }
    }
}

impl Gkat<EncodedFormula> for SATGkat {
    #[inline]
    fn mk_zero(&mut self) -> EncodedFormula {
        EncodedFormula::constant(false)
    }

    #[inline]
    fn mk_one(&mut self) -> EncodedFormula {
        EncodedFormula::constant(true)
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
        if *b1 == self.mk_zero() || *b2 == self.mk_zero() {
            return self.mk_zero();
        } else if *b1 == self.mk_one() {
            return *b2;
        } else if *b2 == self.mk_one() {
            return *b1;
        } else if b1 == b2 {
            return *b1;
        }
        self.man.and(&[*b1, *b2])
    }

    fn mk_or(&mut self, b1: &EncodedFormula, b2: &EncodedFormula) -> EncodedFormula {
        if *b1 == self.mk_zero() {
            return *b2;
        } else if *b2 == self.mk_zero() {
            return *b1;
        } else if *b1 == self.mk_one() || *b2 == self.mk_one() {
            return self.mk_one();
        } else if b1 == b2 {
            return *b1;
        }
        self.man.or(&[*b1, *b2])
    }

    #[inline]
    fn mk_not(&mut self, b: &EncodedFormula) -> EncodedFormula {
        self.man.not(*b)
    }

    fn is_false(&mut self, b: &EncodedFormula) -> bool {
        if b == &self.mk_zero() {
            return true;
        } else if b == &self.mk_one() {
            return false;
        } else if let Some(result) = self.is_false_cache.get(b) {
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

    #[inline]
    fn hashcons(&mut self, e: Exp_<EncodedFormula>) -> Exp<EncodedFormula> {
        self.exp_hcons.mk(e)
    }
}
