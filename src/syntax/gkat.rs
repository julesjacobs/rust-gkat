use super::*;
use crate::parsing;
use ahash::HashMap;
use cudd::{Cudd_Init, CUDD_CACHE_SLOTS, CUDD_UNIQUE_SLOTS};
use hashconsing::HConsign;

pub(super) struct Builder(pub *mut DdManager);

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { Cudd_Quit(self.0) };
    }
}

pub struct Gkat {
    pub(super) name_map: HashMap<String, BExp>,
    pub(super) exp_hcons: HConsign<Exp_>,
    pub(super) bexp_builder: Builder,
}

impl Gkat {
    pub fn new() -> Self {
        unsafe {
            let builder = Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0);
            Gkat {
                name_map: HashMap::default(),
                exp_hcons: HConsign::empty(),
                bexp_builder: Builder(builder),
            }
        }
    }

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
