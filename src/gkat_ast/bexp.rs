use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{self, Hash, Hasher},
};

use builder::BottomUpBuilder;
use hashconsing::{HConsed, HConsign, HashConsign};
use repr::{BddPtr, DDNNFPtr, VarLabel};
use rsdd::*;

#[derive(Debug, Clone, Eq)]
pub struct Name {
    name: String,
    id: u64,
}

pub struct NameBuilder {
    stamp: u64,
    hmap: HashMap<String, u64>,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        NameBuilder {
            stamp: 0,
            hmap: HashMap::new(),
        }
    }

    pub fn mk(&mut self, s: String) -> Name {
        match self.hmap.get(&s) {
            Some(id) => Name { name: s, id: *id },
            None => {
                self.stamp += 1;
                self.hmap.insert(s.clone(), self.stamp);
                Name {
                    name: s,
                    id: self.stamp,
                }
            }
        }
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

pub fn mk_zero(fb: &mut HConsign<BExp_>) -> BExp {
    fb.mk(BExp_::Zero)
}

pub fn mk_one(fb: &mut HConsign<BExp_>) -> BExp {
    fb.mk(BExp_::One)
}

pub fn mk_pbool(nb: &mut NameBuilder, fb: &mut HConsign<BExp_>, s: String) -> BExp {
    fb.mk(BExp_::PBool(nb.mk(s)))
}

pub fn mk_or(fb: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == mk_one(fb) {
        mk_one(fb)
    } else if b2 == mk_one(fb) {
        mk_one(fb)
    } else if b1 == mk_zero(fb) {
        b2
    } else if b2 == mk_zero(fb) {
        b1
    } else if b1 == b2 {
        b1
    } else {
        fb.mk(BExp_::Or(b1, b2))
    }
}

pub fn mk_and(fb: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == mk_one(fb) {
        b2
    } else if b2 == mk_one(fb) {
        b1
    } else if b1 == mk_zero(fb) {
        mk_zero(fb)
    } else if b2 == mk_zero(fb) {
        mk_zero(fb)
    } else if b1 == b2 {
        b1
    } else {
        fb.mk(BExp_::And(b1, b2))
    }
}

pub fn mk_not(fb: &mut HConsign<BExp_>, b1: BExp) -> BExp {
    if b1 == mk_one(fb) {
        mk_zero(fb)
    } else if b1 == mk_zero(fb) {
        mk_one(fb)
    } else {
        fb.mk(BExp_::Not(b1))
    }
}

pub fn to_bdd<'a, Builder>(bdd: &'a Builder, b: &BExp) -> BddPtr<'a>
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use BExp_::*;
    match b.get() {
        One => bdd.true_ptr(),
        Zero => bdd.false_ptr(),
        PBool(n) => bdd.var(VarLabel::new(n.id), true),
        Or(b1, b2) => {
            let b1 = to_bdd(bdd, b1);
            let b2 = to_bdd(bdd, b2);
            bdd.or(b1, b2)
        }
        And(b1, b2) => {
            let b1 = to_bdd(bdd, b1);
            let b2 = to_bdd(bdd, b2);
            bdd.and(b1, b2)
        }
        Not(b) => {
            let b = to_bdd(bdd, b);
            bdd.negate(b)
        }
    }
}

pub fn is_false<'a, Builder>(bdd: &'a Builder, b: &BExp) -> bool
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    let b = to_bdd(bdd, b);
    b.is_false()
}

pub fn is_equiv<'a, Builder>(bdd: &'a Builder, b1: &BExp, b2: &BExp) -> bool
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    let b1 = to_bdd(bdd, b1);
    let b2 = to_bdd(bdd, b2);
    b1 == b2
}
