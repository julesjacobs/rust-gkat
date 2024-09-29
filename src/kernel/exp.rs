use super::*;
use crate::parsing;
use ahash::AHasher;
use disjoint_sets::UnionFindNode;
use hashconsing::{HConsed, HashConsign};
use rsdd::{builder::BottomUpBuilder, repr::BddPtr};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq)]
pub struct Action {
    pub(super) name: String,
    pub(super) id: u64,
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

impl<'a, Builder: BottomUpBuilder<'a, BddPtr<'a>>> GkatManager<'a, Builder> {
    fn mk_action(&mut self, s: String) -> Action {
        let mut hasher = AHasher::default();
        s.hash(&mut hasher);
        let id = hasher.finish();
        Action { name: s, id: id }
    }

    pub fn mk_act(&mut self, s: String) -> Exp {
        let x = self.mk_action(s);
        self.exp_hcons.mk(Exp_::Act(x))
    }

    pub fn mk_skip(&mut self) -> Exp {
        let b = self.mk_one();
        self.mk_test(b)
    }

    pub fn mk_fail(&mut self) -> Exp {
        let b = self.mk_zero();
        self.mk_test(b)
    }

    pub fn mk_test(&mut self, b: BExp) -> Exp {
        self.exp_hcons.mk(Exp_::Test(b))
    }

    pub fn mk_seq(&mut self, p1: Exp, p2: Exp) -> Exp {
        use Exp_::*;
        match (p1.get(), p2.get()) {
            (Test(b1), Test(b2)) => {
                let b1 = b1.clone();
                let b2 = b2.clone();
                let b3 = self.mk_and(b1, b2);
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

    pub fn mk_if(&mut self, b: BExp, p1: Exp, p2: Exp) -> Exp {
        if b == self.mk_one() {
            p1
        } else if b == self.mk_zero() {
            p2
        } else if p1 == self.mk_fail() {
            let nb = self.mk_not(b);
            let p1 = self.mk_test(nb);
            self.mk_seq(p1, p2)
        } else if p2 == self.mk_fail() {
            let p0 = self.mk_test(b);
            self.mk_seq(p0, p1)
        } else {
            self.exp_hcons.mk(Exp_::If(b, p1, p2))
        }
    }

    pub fn mk_while(&mut self, b: BExp, p: Exp) -> Exp {
        self.exp_hcons.mk(Exp_::While(b, p))
    }

    pub fn from_exp(&mut self, raw: parsing::Exp) -> Exp {
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

    pub fn mk_uf(&mut self, exp: &Exp) -> UnionFindNode<()> {
        match self.uf_table.get(exp) {
            Some(node) => node.clone(),
            None => {
                let node = UnionFindNode::new(());
                self.uf_table.insert(exp.clone(), node.clone());
                node
            }
        }
    }
}
