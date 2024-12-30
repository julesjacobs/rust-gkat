use super::*;
use std::{fmt::Debug, hash::Hash, ptr};

pub struct BExp {
    pub(super) cudd: *mut DdManager,
    pub(super) node: *mut DdNode,
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
