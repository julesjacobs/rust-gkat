use super::*;
use core::fmt;
use std::{ffi::CStr, fmt::Debug, hash::Hash};

pub struct BExp {
    pub(super) ctx: Z3_context,
    pub(super) ast: Z3_ast,
}

impl Drop for BExp {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.ctx, self.ast) };
    }
}

impl Clone for BExp {
    fn clone(&self) -> Self {
        unsafe { Z3_inc_ref(self.ctx, self.ast) };
        Self {
            ctx: self.ctx,
            ast: self.ast,
        }
    }
}

impl Debug for BExp {
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

impl Hash for BExp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let id = unsafe { Z3_get_ast_id(self.ctx, self.ast) };
        id.hash(state);
    }
}

impl PartialEq for BExp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Z3_is_eq_ast(self.ctx, self.ast, other.ast) }
    }
}

impl Eq for BExp {
    fn assert_receiver_is_total_eq(&self) {}
}

impl BExp {
    pub fn and(&self, other: &Self) -> Self {
        unsafe {
            let ast = Z3_mk_and(self.ctx, 2, [self.ast, other.ast].as_ptr());
            let ast = Z3_simplify(self.ctx, ast);
            Z3_inc_ref(self.ctx, ast);
            Self {
                ctx: self.ctx,
                ast: ast,
            }
        }
    }

    pub fn or(&self, other: &Self) -> Self {
        unsafe {
            let ast = Z3_mk_or(self.ctx, 2, [self.ast, other.ast].as_ptr());
            let ast = Z3_simplify(self.ctx, ast);
            Z3_inc_ref(self.ctx, ast);
            Self {
                ctx: self.ctx,
                ast: ast,
            }
        }
    }

    pub fn not(&self) -> Self {
        unsafe {
            let ast = Z3_mk_not(self.ctx, self.ast);
            let ast = Z3_simplify(self.ctx, ast);
            Z3_inc_ref(self.ctx, ast);
            Self {
                ctx: self.ctx,
                ast: ast,
            }
        }
    }
}
