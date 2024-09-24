use std::{
    collections::hash_map::DefaultHasher,
    hash::{self, Hash, Hasher},
};

use builder::{
    bdd::{BddBuilder, RobddBuilder},
    cache::AllIteTable,
    BottomUpBuilder,
};
use hashconsing::{HConsed, HConsign, HashConsign};
use petgraph::matrix_graph::Zero;
use repr::{BddPtr, DDNNFPtr, VarLabel};
use rsdd::*;

#[derive(Debug, Clone, Eq)]
pub struct Name {
    name: String,
    id: u64,
}

impl Name {
    pub fn mk(s: String) -> Self {
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

pub fn mk_zero(factory: &mut HConsign<BExp_>) -> BExp {
    factory.mk(BExp_::Zero)
}

pub fn mk_one(factory: &mut HConsign<BExp_>) -> BExp {
    factory.mk(BExp_::One)
}

pub fn mk_pbool(factory: &mut HConsign<BExp_>, s: String) -> BExp {
    factory.mk(BExp_::PBool(Name::mk(s)))
}

pub fn mk_or(factory: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == mk_one(factory) {
        mk_one(factory)
    } else if b2 == mk_one(factory) {
        mk_one(factory)
    } else if b1 == mk_zero(factory) {
        b2
    } else if b2 == mk_zero(factory) {
        b1
    } else if b1 == b2 {
        b1
    } else {
        factory.mk(BExp_::Or(b1, b2))
    }
}

pub fn mk_and(factory: &mut HConsign<BExp_>, b1: BExp, b2: BExp) -> BExp {
    if b1 == mk_one(factory) {
        b2
    } else if b2 == mk_one(factory) {
        b1
    } else if b1 == mk_zero(factory) {
        mk_zero(factory)
    } else if b2 == mk_zero(factory) {
        mk_zero(factory)
    } else if b1 == b2 {
        b1
    } else {
        factory.mk(BExp_::And(b1, b2))
    }
}

pub fn mk_not(factory: &mut HConsign<BExp_>, b1: BExp) -> BExp {
    if b1 == mk_one(factory) {
        mk_zero(factory)
    } else if b1 == mk_zero(factory) {
        mk_one(factory)
    } else {
        factory.mk(BExp_::Not(b1))
    }
}

pub fn to_bdd<'a>(builder: &'a RobddBuilder<'a, AllIteTable<BddPtr<'a>>>, b: &BExp) -> BddPtr<'a> {
    use BExp_::*;
    match b.get() {
        One => builder.true_ptr(),
        Zero => builder.false_ptr(),
        PBool(n) => builder.var(VarLabel::new(n.id), true),
        Or(b1, b2) => {
            let b1 = to_bdd(builder, b1);
            let b2 = to_bdd(builder, b2);
            builder.or(b1, b2)
        }
        And(b1, b2) => {
            let b1 = to_bdd(builder, b1);
            let b2 = to_bdd(builder, b2);
            builder.and(b1, b2)
        }
        Not(b) => {
            let b = to_bdd(builder, b);
            builder.negate(b)
        }
    }
}

pub fn is_false<'a>(builder: &'a RobddBuilder<'a, AllIteTable<BddPtr<'a>>>, b: &BExp) -> bool {
    let b = to_bdd(builder, b);
    b.is_false()
}

pub fn is_equiv<'a>(
    builder: &'a RobddBuilder<'a, AllIteTable<BddPtr<'a>>>,
    b1: &BExp,
    b2: &BExp,
) -> bool {
    let b1 = to_bdd(builder, b1);
    let b2 = to_bdd(builder, b2);
    b1 == b2
}
