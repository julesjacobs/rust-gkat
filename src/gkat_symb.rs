extern crate hashconsing;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{self, Hash, Hasher},
};

use hashconsing::{HConsed, HConsign, HashConsign};
use petgraph::matrix_graph::Zero;

#[derive(Debug, Clone, Eq)]
struct Name {
    name: String,
    id: u64,
}

impl Name {
    fn mk(s: String) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let id = hasher.finish();
        Name { name: s, id: id }
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

pub fn zero(factory: &mut HConsign<BExp_>) -> BExp {
    factory.mk(BExp_::Zero)
}

pub fn one(factory: &mut HConsign<BExp_>) -> BExp {
    factory.mk(BExp_::One)
}

pub fn pbool(factory: &mut HConsign<BExp_>, s: String) -> BExp {
    factory.mk(BExp_::PBool(Name::mk(s)))
}

pub fn or(factory: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == one(factory) {
        one(factory)
    } else if b2 == one(factory) {
        one(factory)
    } else if b1 == zero(factory) {
        b2
    } else if b2 == zero(factory) {
        b1
    } else if b1 == b2 {
        b1
    } else {
        factory.mk(BExp_::Or(b1, b2))
    }
}

pub fn and(factory: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == one(factory) {
        b2
    } else if b2 == one(factory) {
        b1
    } else if b1 == zero(factory) {
        zero(factory)
    } else if b2 == zero(factory) {
        zero(factory)
    } else if b1 == b2 {
        b1
    } else {
        factory.mk(BExp_::And(b1, b2))
    }
}

pub fn not(factory: &mut HConsign<BExp_>, b1: BExp) -> BExp {
    if b1 == one(factory) {
        zero(factory)
    } else if b1 == zero(factory) {
        one(factory)
    } else {
        factory.mk(BExp_::Not(b1))
    }
}
