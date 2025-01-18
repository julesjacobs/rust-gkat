use super::*;
use ahash::HashMap;
use core::fmt;
use hashconsing::{HConsign, HashConsign};
use std::{ffi::CStr, fmt::Debug, hash::Hash, ptr};

// BExp based on Z3.
pub struct Z3BExp {
    ctx: Z3_context,
    ast: Z3_ast,
}

impl Drop for Z3BExp {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.ctx, self.ast) };
    }
}

impl BExp for Z3BExp {}

impl Clone for Z3BExp {
    fn clone(&self) -> Self {
        unsafe { Z3_inc_ref(self.ctx, self.ast) };
        Self {
            ctx: self.ctx,
            ast: self.ast,
        }
    }
}

impl Debug for Z3BExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let p = unsafe { Z3_ast_to_string(self.ctx, self.ast) };
        if p.is_null() {
            return Result::Err(fmt::Error);
        }
        match unsafe { CStr::from_ptr(p) }.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Result::Err(fmt::Error),
        }
    }
}

impl Hash for Z3BExp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let id = unsafe { Z3_get_ast_id(self.ctx, self.ast) };
        id.hash(state);
    }
}

impl PartialEq for Z3BExp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Z3_is_eq_ast(self.ctx, self.ast, other.ast) }
    }
}

impl Eq for Z3BExp {
    fn assert_receiver_is_total_eq(&self) {}
}

// Gkat based on Z3.
pub(super) struct Z3Manager {
    ctx: Z3_context,
    solver: Z3_solver,
}

impl Drop for Z3Manager {
    fn drop(&mut self) {
        unsafe {
            Z3_solver_dec_ref(self.ctx, self.solver);
            Z3_del_context(self.ctx);
        };
    }
}

impl Z3Manager {
    pub fn new() -> Self {
        unsafe {
            let cfg = Z3_mk_config();
            Z3_set_param_value(cfg, c"proof".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"debug_ref_count".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"trace".as_ptr(), c"false".as_ptr());
            Z3_set_param_value(cfg, c"well_sorted_check".as_ptr(), c"false".as_ptr());
            let ctx = Z3_mk_context(cfg);
            let solver = Z3_mk_simple_solver(ctx);
            Z3_solver_inc_ref(ctx, solver);
            Self {
                ctx: ctx,
                solver: solver,
            }
        }
    }
}

pub struct Z3Gkat {
    name_map: HashMap<String, Z3BExp>,
    exp_hcons: HConsign<Exp_<Z3BExp>>,
    // caching
    is_true_cache: HashMap<Z3BExp, bool>,
    is_false_cache: HashMap<Z3BExp, bool>,
    // HACK: drop last
    man: Z3Manager,
}

impl Z3Gkat {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::default(),
            exp_hcons: HConsign::empty(),
            // caching
            is_true_cache: HashMap::default(),
            is_false_cache: HashMap::default(),
            // builder
            man: Z3Manager::new(),
        }
    }
}

impl Gkat<Z3BExp> for Z3Gkat {
    fn mk_zero(&mut self) -> Z3BExp {
        unsafe {
            let ast = Z3_mk_false(self.man.ctx);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        }
    }

    fn mk_one(&mut self) -> Z3BExp {
        unsafe {
            let ast = Z3_mk_true(self.man.ctx);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        }
    }

    fn mk_var(&mut self, s: String) -> Z3BExp {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let x = unsafe {
            let srt = Z3_mk_bool_sort(self.man.ctx);
            let ast = Z3_mk_fresh_const(self.man.ctx, ptr::null(), srt);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        };
        self.name_map.insert(s, x.clone());
        return x;
    }

    fn mk_and(&mut self, b1: &Z3BExp, b2: &Z3BExp) -> Z3BExp {
        unsafe {
            let ast = Z3_mk_and(self.man.ctx, 2, [b1.ast, b2.ast].as_ptr());
            let ast = Z3_simplify(self.man.ctx, ast);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        }
    }

    fn mk_or(&mut self, b1: &Z3BExp, b2: &Z3BExp) -> Z3BExp {
        unsafe {
            let ast = Z3_mk_or(self.man.ctx, 2, [b1.ast, b2.ast].as_ptr());
            let ast = Z3_simplify(self.man.ctx, ast);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        }
    }

    fn mk_not(&mut self, b: &Z3BExp) -> Z3BExp {
        unsafe {
            let ast = Z3_mk_not(self.man.ctx, b.ast);
            let ast = Z3_simplify(self.man.ctx, ast);
            Z3_inc_ref(self.man.ctx, ast);
            Z3BExp {
                ctx: self.man.ctx,
                ast: ast,
            }
        }
    }

    fn is_true(&mut self, b: &Z3BExp) -> bool {
        if let Some(b) = self.is_true_cache.get(b) {
            return *b;
        }
        let ast = self.mk_not(b).ast;
        let result = match unsafe {
            Z3_solver_check_assumptions(self.man.ctx, self.man.solver, 1, [ast].as_ptr())
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

    fn is_false(&mut self, b: &Z3BExp) -> bool {
        if let Some(b) = self.is_false_cache.get(b) {
            return *b;
        }
        let result = match unsafe {
            let ast = b.ast;
            Z3_solver_check_assumptions(self.man.ctx, self.man.solver, 1, [ast].as_ptr())
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

    fn is_equiv(&mut self, b1: &Z3BExp, b2: &Z3BExp) -> bool {
        unsafe {
            let ast = Z3_mk_iff(self.man.ctx, b1.ast, b2.ast);
            let ast = Z3_mk_not(self.man.ctx, ast);
            match Z3_solver_check_assumptions(self.man.ctx, self.man.solver, 1, [ast].as_ptr()) {
                Z3_L_FALSE => true,
                Z3_L_UNDEF => panic!("unknown"),
                Z3_L_TRUE => false,
                _ => unreachable!(),
            }
        }
    }

    fn hashcons(&mut self, e: Exp_<Z3BExp>) -> Exp<Z3BExp> {
        self.exp_hcons.mk(e)
    }
}
