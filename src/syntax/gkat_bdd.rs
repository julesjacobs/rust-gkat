use super::*;
use ahash::HashMap;
use core::fmt;
use cudd::{Cudd_Init, CUDD_CACHE_SLOTS, CUDD_UNIQUE_SLOTS};
use hashconsing::{HConsign, HashConsign};
use std::{fmt::Debug, hash::Hash, ptr};

// BExp based on BDD.
pub struct BDDBExp {
    cudd: *mut DdManager,
    node: *mut DdNode,
}

impl Drop for BDDBExp {
    fn drop(&mut self) {
        unsafe { Cudd_RecursiveDeref(self.cudd, self.node) };
    }
}

impl BExp for BDDBExp {}

impl Clone for BDDBExp {
    fn clone(&self) -> Self {
        unsafe { Cudd_Ref(self.node) };
        Self {
            cudd: self.cudd,
            node: self.node,
        }
    }
}

impl Debug for BDDBExp {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { Cudd_PrintMinterm(self.cudd, self.node) };
        std::fmt::Result::Ok(())
    }
}

impl Hash for BDDBExp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self, state);
    }
}

impl PartialEq for BDDBExp {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for BDDBExp {
    fn assert_receiver_is_total_eq(&self) {}
}

// Gkat based on BDD.
struct BDDManager(*mut DdManager);

impl Drop for BDDManager {
    fn drop(&mut self) {
        unsafe { Cudd_Quit(self.0) };
    }
}

impl BDDManager {
    fn new() -> Self {
        let man = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        Self(man)
    }
}

pub struct BDDGkat {
    name_map: HashMap<String, BDDBExp>,
    exp_hcons: HConsign<Exp_<BDDBExp>>,
    man: BDDManager,
}

impl BDDGkat {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::default(),
            exp_hcons: HConsign::empty(),
            man: BDDManager::new(),
        }
    }
}

impl Gkat<BDDBExp> for BDDGkat {
    fn mk_zero(&mut self) -> BDDBExp {
        unsafe {
            let node = Cudd_ReadLogicZero(self.man.0);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        }
    }

    fn mk_one(&mut self) -> BDDBExp {
        unsafe {
            let node = Cudd_ReadOne(self.man.0);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        }
    }

    fn mk_var(&mut self, s: String) -> BDDBExp {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }
        let x = unsafe {
            let node = Cudd_bddNewVar(self.man.0);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        };
        self.name_map.insert(s, x.clone());
        return x;
    }

    fn mk_and(&mut self, b1: &BDDBExp, b2: &BDDBExp) -> BDDBExp {
        unsafe {
            let node = Cudd_bddAnd(self.man.0, b1.node, b2.node);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        }
    }

    fn mk_or(&mut self, b1: &BDDBExp, b2: &BDDBExp) -> BDDBExp {
        unsafe {
            let node = Cudd_bddOr(self.man.0, b1.node, b2.node);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        }
    }

    fn mk_not(&mut self, b: &BDDBExp) -> BDDBExp {
        unsafe {
            let node = Cudd_Not(b.node);
            Cudd_Ref(node);
            BDDBExp {
                cudd: self.man.0,
                node: node,
            }
        }
    }

    fn is_true(&mut self, b: &BDDBExp) -> bool {
        unsafe { b.node == Cudd_ReadOne(self.man.0) }
    }

    fn is_false(&mut self, b: &BDDBExp) -> bool {
        unsafe { b.node == Cudd_ReadLogicZero(self.man.0) }
    }

    fn is_equiv(&mut self, b1: &BDDBExp, b2: &BDDBExp) -> bool {
        b1 == b2
    }

    fn hashcons(&mut self, e: Exp_<BDDBExp>) -> Exp<BDDBExp> {
        self.exp_hcons.mk(e)
    }
}
