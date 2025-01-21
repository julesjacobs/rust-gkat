use cadical_sys::*;
use gxhash::{GxBuildHasher, HashMap};
use hashconsing::{HConsed, HConsign, HashConsign};

use super::{BExp, Exp, Exp_, Gkat};

pub type Formula = HConsed<Formula_>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Formula_ {
    Cst(bool),
    Lit(i32),
    Cnj(Formula, Formula),
    Dsj(Formula, Formula),
    Eqv(Formula, Formula),
    Not(Formula),
}

impl BExp for Formula {}

pub struct CADGkat {
    // formula manager
    var_stamp: i32,
    formula_hcons: HConsign<Formula_>,
    solver: CaDiCal,
    // exp manager
    name_map: HashMap<String, Formula>,
    exp_hcons: HConsign<Exp_<Formula>, GxBuildHasher>,
    // caching
    cnf_ctrl_cache: HashMap<Formula, i32>,
    is_false_cache: HashMap<Formula, bool>,
}

impl CADGkat {
    pub fn new() -> Self {
        Self {
            var_stamp: 0,
            formula_hcons: HConsign::empty(),
            solver: CaDiCal::new(),
            name_map: HashMap::default(),
            exp_hcons: HConsign::with_hasher(GxBuildHasher::default()),
            cnf_ctrl_cache: HashMap::default(),
            is_false_cache: HashMap::default(),
        }
    }

    pub fn fresh_lit(&mut self) -> i32 {
        self.var_stamp += 1;
        return self.var_stamp;
    }

    pub fn pg_cnf(&mut self, b: &Formula) -> i32 {
        if let Formula_::Lit(l) = b.get() {
            return *l;
        }
        if let Some(l) = self.cnf_ctrl_cache.get(b) {
            return *l;
        }
        let l = self.fresh_lit();
        self.pg_cnf_rec(b, l);
        return l;
    }

    pub fn pg_cnf_rec(&mut self, b: &Formula, l: i32) {
        use Formula_::*;
        match b.get() {
            Cst(_) => unreachable!(),
            Lit(_) => (),
            Cnj(b1, b2) => {
                let l1 = self.pg_cnf(b1);
                let l2 = self.pg_cnf(b2);
                self.solver.clause2(-l, l1);
                self.solver.clause2(-l, l2);
                self.solver.clause3(-l1, -l2, l);
            }
            Dsj(b1, b2) => {
                let l1 = self.pg_cnf(b1);
                let l2 = self.pg_cnf(b2);
                self.solver.clause3(-l, l1, l2);
                self.solver.clause2(-l1, l);
                self.solver.clause2(-l2, l);
            }
            Eqv(b1, b2) => {
                let l1 = self.pg_cnf(b1);
                let l2 = self.pg_cnf(b2);
                self.solver.clause3(-l, -l1, l2);
                self.solver.clause3(-l, l1, -l2);
                self.solver.clause1(l);
                self.solver.clause2(l1, l2);
                self.solver.clause2(-l1, -l2);
            }
            Not(b1) => self.pg_cnf_rec(b1, -l),
        };
        self.cnf_ctrl_cache.insert(b.clone(), l);
    }

    pub fn check_sat(&mut self, b: &Formula) -> bool {
        if let Formula_::Cst(b) = b.get() {
            return *b;
        }
        let ctrl = self.pg_cnf(b);
        self.solver.assume(ctrl);
        let result = match self.solver.solve() {
            Status::SATISFIABLE => true,
            Status::UNSATISFIABLE => false,
            Status::UNKNOWN => panic!(),
        };
        return result;
    }
}

// Gkat based on BDD.
impl Gkat<Formula> for CADGkat {
    #[inline]
    fn mk_zero(&mut self) -> Formula {
        self.formula_hcons.mk(Formula_::Cst(false))
    }

    #[inline]
    fn mk_one(&mut self) -> Formula {
        self.formula_hcons.mk(Formula_::Cst(true))
    }

    fn mk_var(&mut self, s: String) -> Formula {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let l = self.fresh_lit();
        let x = self.formula_hcons.mk(Formula_::Lit(l));
        self.name_map.insert(s, x.clone());
        return x;
    }

    fn mk_and(&mut self, b1: &Formula, b2: &Formula) -> Formula {
        use Formula_::*;
        match (b1.get(), b2.get()) {
            (Cst(true), _) => b2.clone(),
            (_, Cst(true)) => b1.clone(),
            (Cst(false), _) => self.mk_zero(),
            (_, Cst(false)) => self.mk_zero(),
            _ => self.formula_hcons.mk(Cnj(b1.clone(), b2.clone())),
        }
    }

    fn mk_or(&mut self, b1: &Formula, b2: &Formula) -> Formula {
        use Formula_::*;
        match (b1.get(), b2.get()) {
            (Cst(true), _) => self.mk_one(),
            (_, Cst(true)) => self.mk_one(),
            (Cst(false), _) => b2.clone(),
            (_, Cst(false)) => b1.clone(),
            _ => self.formula_hcons.mk(Dsj(b1.clone(), b2.clone())),
        }
    }

    fn mk_not(&mut self, b: &Formula) -> Formula {
        use Formula_::*;
        match b.get() {
            Cst(b) => self.formula_hcons.mk(Formula_::Cst(!b)),
            Lit(l) => self.formula_hcons.mk(Lit(-l)),
            Not(b) => b.clone(),
            _ => self.formula_hcons.mk(Not(b.clone())),
        }
    }

    fn is_false(&mut self, b: &Formula) -> bool {
        if let Some(result) = self.is_false_cache.get(b) {
            return *result;
        }
        let result = !self.check_sat(b);
        self.is_false_cache.insert(b.clone(), result);
        return result;
    }

    fn is_equiv(&mut self, b1: &Formula, b2: &Formula) -> bool {
        use Formula_::*;
        let b = match (b1.get(), b2.get()) {
            (Cst(true), _) => b2.clone(),
            (_, Cst(true)) => b1.clone(),
            (Cst(false), _) => self.mk_not(b2),
            (_, Cst(false)) => self.mk_not(b1),
            _ if b1 == b2 => self.mk_one(),
            _ => self.formula_hcons.mk(Formula_::Eqv(b1.clone(), b2.clone())),
        };
        let nb = self.mk_not(&b);
        self.is_false(&nb)
    }

    #[inline]
    fn hashcons(&mut self, e: Exp_<Formula>) -> Exp<Formula> {
        self.exp_hcons.mk(e)
    }
}
