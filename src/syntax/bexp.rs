use super::*;
use crate::parsing;
use std::{fmt::Debug, hash::Hash, ptr};

pub struct BExp {
    cudd: *mut DdManager,
    node: *mut DdNode,
}

impl Drop for BExp {
    fn drop(&mut self) {
        unsafe { Cudd_RecursiveDeref(self.cudd, self.node) };
    }
}

impl Clone for BExp {
    fn clone(&self) -> Self {
        unsafe { Cudd_Ref(self.node) };
        Self {
            cudd: self.cudd,
            node: self.node,
        }
    }
}

impl Debug for BExp {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { Cudd_PrintMinterm(self.cudd, self.node) };
        std::fmt::Result::Ok(())
    }
}

impl Hash for BExp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self, state);
    }
}

impl PartialEq for BExp {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for BExp {
    fn assert_receiver_is_total_eq(&self) {}
}

impl BExp {
    pub fn and(&self, other: &Self) -> Self {
        unsafe {
            let node = Cudd_bddAnd(self.cudd, self.node, other.node);
            Cudd_Ref(node);
            Self {
                cudd: self.cudd,
                node: node,
            }
        }
    }

    pub fn or(&self, other: &Self) -> Self {
        unsafe {
            let node = Cudd_bddOr(self.cudd, self.node, other.node);
            Cudd_Ref(node);
            Self {
                cudd: self.cudd,
                node: node,
            }
        }
    }

    pub fn not(&self) -> Self {
        unsafe {
            let node = Cudd_Not(self.node);
            Cudd_Ref(node);
            Self {
                cudd: self.cudd,
                node: node,
            }
        }
    }

    pub fn is_true(&self) -> bool {
        unsafe { self.node == Cudd_ReadOne(self.cudd) }
    }

    pub fn is_false(&self) -> bool {
        unsafe { self.node == Cudd_ReadLogicZero(self.cudd) }
    }
}

impl Gkat {
    pub fn zero(&mut self) -> BExp {
        unsafe {
            let node = Cudd_ReadLogicZero(self.bexp_builder.0);
            Cudd_Ref(node);
            BExp {
                cudd: self.bexp_builder.0,
                node: node,
            }
        }
    }

    pub fn one(&mut self) -> BExp {
        unsafe {
            let node = Cudd_ReadOne(self.bexp_builder.0);
            Cudd_Ref(node);
            BExp {
                cudd: self.bexp_builder.0,
                node: node,
            }
        }
    }

    pub fn var(&mut self, s: String) -> BExp {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let x = unsafe {
            let node = Cudd_bddNewVar(self.bexp_builder.0);
            Cudd_Ref(node);
            BExp {
                cudd: self.bexp_builder.0,
                node: node,
            }
        };
        self.name_map.insert(s, x.clone());
        return x;
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
