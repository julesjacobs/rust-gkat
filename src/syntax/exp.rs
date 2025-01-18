use super::*;
use crate::parsing;
use ahash::AHasher;
use hashconsing::{HConsed, HashConsign};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub type Exp = HConsed<Exp_>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Exp_ {
    Act(u64),
    Seq(Exp, Exp),
    Ifte(BExp, Exp, Exp),
    Test(BExp),
    While(BExp, Exp),
}

impl Gkat {
    pub fn mk_act(&mut self, s: String) -> Exp {
        let mut hasher = AHasher::default();
        s.hash(&mut hasher);
        let a = hasher.finish();
        self.exp_hcons.mk(Exp_::Act(a))
    }

    pub fn mk_skip(&mut self) -> Exp {
        let b = self.one();
        self.mk_test(b)
    }

    pub fn mk_fail(&mut self) -> Exp {
        let b = self.zero();
        self.mk_test(b)
    }

    pub fn mk_test(&mut self, b: BExp) -> Exp {
        self.exp_hcons.mk(Exp_::Test(b))
    }

    pub fn mk_seq(&mut self, p1: Exp, p2: Exp) -> Exp {
        use Exp_::*;
        match (p1.get(), p2.get()) {
            (Test(b1), Test(b2)) => {
                let b3 = b1.and(b2);
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

    pub fn mk_ifte(&mut self, b: BExp, p1: Exp, p2: Exp) -> Exp {
        if self.is_true(&b) {
            p1
        } else if self.is_false(&b) {
            p2
        } else if p1 == self.mk_fail() {
            let nb = b.not();
            let p1 = self.mk_test(nb);
            self.mk_seq(p1, p2)
        } else if p2 == self.mk_fail() {
            let p0 = self.mk_test(b);
            self.mk_seq(p0, p1)
        } else {
            self.exp_hcons.mk(Exp_::Ifte(b, p1, p2))
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
            Ifte(b, p1, p2) => {
                let b = self.from_bexp(b);
                let p1 = self.from_exp(*p1);
                let p2 = self.from_exp(*p2);
                self.mk_ifte(b, p1, p2)
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
