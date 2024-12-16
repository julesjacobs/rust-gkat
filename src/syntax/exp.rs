use super::*;
use crate::parsing;
use ahash::AHasher;
use hashconsing::{HConsed, HashConsign};
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq)]
pub struct Action {
    name: String,
    id: u64,
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

pub type Exp<BExp> = HConsed<Exp_<BExp>>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Exp_<BExp> {
    Act(Action),
    Seq(Exp<BExp>, Exp<BExp>),
    If(BExp, Exp<BExp>, Exp<BExp>),
    Test(BExp),
    While(BExp, Exp<BExp>),
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> Gkat<'a, Ptr, Builder> {
    fn mk_action(&mut self, s: String) -> Action {
        let mut hasher = AHasher::default();
        s.hash(&mut hasher);
        let id = hasher.finish();
        Action { name: s, id: id }
    }

    pub fn mk_act(&mut self, s: String) -> Exp<Ptr> {
        let x = self.mk_action(s);
        self.exp_hcons.mk(Exp_::Act(x))
    }

    pub fn mk_skip(&mut self) -> Exp<Ptr> {
        let b = self.bexp_builder.true_ptr();
        self.mk_test(b)
    }

    pub fn mk_fail(&mut self) -> Exp<Ptr> {
        let b = self.bexp_builder.false_ptr();
        self.mk_test(b)
    }

    pub fn mk_test(&mut self, b: Ptr) -> Exp<Ptr> {
        self.exp_hcons.mk(Exp_::Test(b))
    }

    pub fn mk_seq(&mut self, p1: Exp<Ptr>, p2: Exp<Ptr>) -> Exp<Ptr> {
        use Exp_::*;
        match (p1.get(), p2.get()) {
            (Test(b1), Test(b2)) => {
                let b1 = b1.clone();
                let b2 = b2.clone();
                let b3 = self.bexp_builder.and(b1, b2);
                self.mk_test(b3)
            }
            _ => {
                if p1 == self.mk_skip() {
                    p2
                } else if p2 == self.mk_skip() {
                    p1
                } else if p1 == self.mk_fail() {
                    self.mk_fail()
                } else if p2 == self.mk_fail() {
                    self.mk_fail()
                } else {
                    self.exp_hcons.mk(Exp_::Seq(p1, p2))
                }
            }
        }
    }

    pub fn mk_if(&mut self, b: Ptr, p1: Exp<Ptr>, p2: Exp<Ptr>) -> Exp<Ptr> {
        if b.is_true() {
            p1
        } else if b.is_false() {
            p2
        } else if p1 == self.mk_fail() {
            let nb = self.bexp_builder.negate(b);
            let p1 = self.mk_test(nb);
            self.mk_seq(p1, p2)
        } else if p2 == self.mk_fail() {
            let p0 = self.mk_test(b);
            self.mk_seq(p0, p1)
        } else {
            self.exp_hcons.mk(Exp_::If(b, p1, p2))
        }
    }

    pub fn mk_while(&mut self, b: Ptr, p: Exp<Ptr>) -> Exp<Ptr> {
        self.exp_hcons.mk(Exp_::While(b, p))
    }

    pub fn from_exp(&mut self, raw: parsing::Exp) -> Exp<Ptr> {
        use parsing::Exp::*;
        match raw {
            Act(s) => self.mk_act(s),
            Seq(p1, p2) => {
                let p1 = self.from_exp(*p1);
                let p2 = self.from_exp(*p2);
                self.mk_seq(p1, p2)
            }
            If(b, p1, p2) => {
                let b = self.from_bexp(b);
                let p1 = self.from_exp(*p1);
                let p2 = self.from_exp(*p2);
                self.mk_if(b, p1, p2)
            }
            Test(b) => {
                let b = self.from_bexp(b);
                self.mk_test(b)
            }
            While(b, p) => {
                let b = self.from_bexp(b);
                let p = self.from_exp(*p);
                self.mk_while(b, p)
            }
        }
    }
}
