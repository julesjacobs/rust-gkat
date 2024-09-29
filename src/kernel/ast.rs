use hashconsing::HConsed;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq)]
pub struct Name {
    pub(super) name: String,
    pub(super) id: u64,
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

#[derive(Clone, Eq)]
pub struct Action {
    pub(super) name: String,
    pub(super) id: u64,
}

impl Action {
    pub fn new(s: String, x: u64) -> Self {
        Action { name: s, id: x }
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Action").field(&self.name).finish()
    }
}

impl PartialEq for Action {
    fn eq(&self, rhs: &Action) -> bool {
        self.id == rhs.id
    }
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.id);
    }
}

pub type Exp = HConsed<Exp_>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Exp_ {
    Act(Action),
    Seq(Exp, Exp),
    If(BExp, Exp, Exp),
    Test(BExp),
    While(BExp, Exp),
}
