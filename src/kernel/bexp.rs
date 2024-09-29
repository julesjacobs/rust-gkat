use super::*;
use crate::parsing;
use hashconsing::{HConsed, HashConsign};
use rsdd::{
    builder::BottomUpBuilder,
    repr::{BddPtr, DDNNFPtr, VarLabel},
};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq)]
pub struct Name {
    name: String,
    id: u64,
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Name").field(&self.name).finish()
    }
}

impl PartialEq for Name {
    fn eq(&self, rhs: &Name) -> bool {
        self.id == rhs.id
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.id);
    }
}

pub type BExp = HConsed<BExp_>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum BExp_ {
    Zero,
    One,
    PBool(Name),
    Or(BExp, BExp),
    And(BExp, BExp),
    Not(BExp),
}

impl<'a, Builder: BottomUpBuilder<'a, BddPtr<'a>>> GkatManager<'a, Builder> {
    fn mk_name(&mut self, s: String) -> Name {
        match self.name_map.get(&s) {
            Some(id) => Name { name: s, id: *id },
            None => {
                self.name_stamp += 1;
                self.name_map.insert(s.clone(), self.name_stamp);
                Name {
                    name: s,
                    id: self.name_stamp,
                }
            }
        }
    }

    pub fn mk_zero(&mut self) -> BExp {
        self.bexp_hcons.mk(BExp_::Zero)
    }

    pub fn mk_one(&mut self) -> BExp {
        self.bexp_hcons.mk(BExp_::One)
    }

    pub fn mk_pbool(&mut self, s: String) -> BExp {
        let x = self.mk_name(s);
        self.bexp_hcons.mk(BExp_::PBool(x))
    }

    pub fn mk_or(&mut self, b1: BExp, b2: BExp) -> BExp {
        let one = self.mk_one();
        let zero = self.mk_zero();
        if b1 == one.clone() {
            one
        } else if b2 == one.clone() {
            one
        } else if b1 == zero {
            b2
        } else if b2 == zero {
            b1
        } else if b1 == b2 {
            b1
        } else {
            self.bexp_hcons.mk(BExp_::Or(b1, b2))
        }
    }

    pub fn mk_and(&mut self, b1: BExp, b2: BExp) -> BExp {
        let one = self.mk_one();
        let zero = self.mk_zero();
        if b1 == one {
            b2
        } else if b2 == one {
            b1
        } else if b1 == zero {
            zero
        } else if b2 == zero {
            zero
        } else if b1 == b2 {
            b1
        } else {
            self.bexp_hcons.mk(BExp_::And(b1, b2))
        }
    }

    pub fn mk_not(&mut self, b1: BExp) -> BExp {
        let one = self.mk_one();
        let zero = self.mk_zero();
        if b1 == one {
            zero
        } else if b1 == zero {
            one
        } else {
            self.bexp_hcons.mk(BExp_::Not(b1))
        }
    }

    pub fn from_bexp(&mut self, raw: parsing::BExp) -> BExp {
        use parsing::BExp::*;
        match raw {
            Zero => self.mk_zero(),
            One => self.mk_one(),
            PBool(s) => self.mk_pbool(s),
            Or(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                self.mk_or(b1, b2)
            }
            And(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                self.mk_and(b1, b2)
            }
            Not(b) => {
                let b = self.from_bexp(*b);
                self.mk_not(b)
            }
        }
    }

    pub fn to_bdd(&mut self, bexp: &BExp) -> BddPtr<'a> {
        use BExp_::*;
        match self.bexp_cache.get(bexp) {
            Some(b) => *b,
            None => {
                let b = match bexp.get() {
                    One => self.bexp_builder.true_ptr(),
                    Zero => self.bexp_builder.false_ptr(),
                    PBool(n) => self.bexp_builder.var(VarLabel::new(n.id), true),
                    Or(b1, b2) => {
                        let b1 = self.to_bdd(b1);
                        let b2 = self.to_bdd(b2);
                        self.bexp_builder.or(b1, b2)
                    }
                    And(b1, b2) => {
                        let b1 = self.to_bdd(b1);
                        let b2 = self.to_bdd(b2);
                        self.bexp_builder.and(b1, b2)
                    }
                    Not(b) => {
                        let b = self.to_bdd(b);
                        self.bexp_builder.negate(b)
                    }
                };
                self.bexp_cache.insert(bexp.clone(), b);
                b
            }
        }
    }

    pub fn is_false(&mut self, bexp: &BExp) -> bool {
        let b = self.to_bdd(bexp);
        self.bexp_cache.insert(bexp.clone(), b);
        b.is_false()
    }

    pub fn is_equiv(&mut self, bexp1: &BExp, bexp2: &BExp) -> bool {
        let b1 = self.to_bdd(bexp1);
        let b2 = self.to_bdd(bexp2);
        b1 == b2
    }
}
