use super::*;
use crate::parsing;
use ahash::HashMap;
use hashconsing::HConsign;
use std::ptr;

pub(super) struct Builder {
    ctx: Z3_context,
    srt: Z3_sort,
    solver: Z3_solver,
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            Z3_solver_dec_ref(self.ctx, self.solver);
            Z3_del_context(self.ctx);
        };
    }
}

impl Builder {
    pub fn new() -> Self {
        unsafe {
            let cfg = Z3_mk_config();
            Z3_set_param_value(cfg, c"proof".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"debug_ref_count".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"trace".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"well_sorted_check".as_ptr(), c"false".as_ptr());
            let ctx = Z3_mk_context(cfg);
            let srt = Z3_mk_bool_sort(ctx);
            let solver = Z3_mk_solver(ctx);
            Z3_solver_inc_ref(ctx, solver);
            Builder {
                ctx: ctx,
                srt: srt,
                solver: solver,
            }
        }
    }
}

pub struct Gkat {
    pub(super) name_map: HashMap<String, BExp>,
    pub(super) exp_hcons: HConsign<Exp_>,
    // caching
    is_true_cache: HashMap<BExp, bool>,
    is_false_cache: HashMap<BExp, bool>,
    // builder (HACK: drop last)
    bexp_builder: Builder,
}

impl Gkat {
    pub fn new() -> Self {
        Gkat {
            name_map: HashMap::default(),
            exp_hcons: HConsign::empty(),
            // caching
            is_true_cache: HashMap::default(),
            is_false_cache: HashMap::default(),
            // builder
            bexp_builder: Builder::new(),
        }
    }

    pub fn zero(&mut self) -> BExp {
        unsafe {
            let ast = Z3_mk_false(self.bexp_builder.ctx);
            Z3_inc_ref(self.bexp_builder.ctx, ast);
            BExp {
                ctx: self.bexp_builder.ctx,
                ast: ast,
            }
        }
    }

    pub fn one(&mut self) -> BExp {
        unsafe {
            let ast = Z3_mk_true(self.bexp_builder.ctx);
            Z3_inc_ref(self.bexp_builder.ctx, ast);
            BExp {
                ctx: self.bexp_builder.ctx,
                ast: ast,
            }
        }
    }

    pub fn var(&mut self, s: String) -> BExp {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let x = unsafe {
            let ast = Z3_mk_fresh_const(self.bexp_builder.ctx, ptr::null(), self.bexp_builder.srt);
            Z3_inc_ref(self.bexp_builder.ctx, ast);
            BExp {
                ctx: self.bexp_builder.ctx,
                ast: ast,
            }
        };
        self.name_map.insert(s, x.clone());
        return x;
    }

    pub fn is_true(&mut self, b: &BExp) -> bool {
        if let Some(b) = self.is_true_cache.get(b) {
            return *b;
        }
        let result = match unsafe {
            Z3_solver_check_assumptions(
                self.bexp_builder.ctx,
                self.bexp_builder.solver,
                1,
                [b.not().ast].as_ptr(),
            )
        } {
            Z3_L_FALSE => true,
            Z3_L_UNDEF => panic!("unknown"),
            Z3_L_TRUE => false,
            _ => unreachable!(),
        };
        self.is_true_cache.insert(b.clone(), result);
        if result {
            self.is_false_cache.insert(b.clone(), false);
        }
        return result;
    }

    pub fn is_false(&mut self, b: &BExp) -> bool {
        if let Some(b) = self.is_false_cache.get(b) {
            return *b;
        }
        let result = match unsafe {
            Z3_solver_check_assumptions(
                self.bexp_builder.ctx,
                self.bexp_builder.solver,
                1,
                [b.ast].as_ptr(),
            )
        } {
            Z3_L_TRUE => false,
            Z3_L_UNDEF => panic!("unknown"),
            Z3_L_FALSE => true,
            _ => unreachable!(),
        };
        self.is_false_cache.insert(b.clone(), result);
        if result {
            self.is_true_cache.insert(b.clone(), false);
        }
        return result;
    }

    pub fn is_equiv(&mut self, lhs: &BExp, rhs: &BExp) -> bool {
        unsafe {
            let ast = Z3_mk_iff(self.bexp_builder.ctx, lhs.ast, rhs.ast);
            let ast = Z3_mk_not(self.bexp_builder.ctx, ast);
            let ast = Z3_simplify(self.bexp_builder.ctx, ast);
            match Z3_solver_check_assumptions(
                self.bexp_builder.ctx,
                self.bexp_builder.solver,
                1,
                [ast].as_ptr(),
            ) {
                Z3_L_FALSE => true,
                Z3_L_UNDEF => panic!("unknown"),
                Z3_L_TRUE => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn from_bexp(&mut self, raw: parsing::BExp) -> BExp {
        use parsing::BExp::*;
        match raw {
            Zero => self.zero(),
            One => self.one(),
            PBool(s) => self.var(s),
            Or(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                b1.or(&b2)
            }
            And(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                b1.and(&b2)
            }
            Not(b) => {
                let b = self.from_bexp(*b);
                b.not()
            }
        }
    }
}
