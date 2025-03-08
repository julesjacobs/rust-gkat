use crate::parsing::{self};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// Trait for generic BExp.
pub trait BExp: Debug + Clone + Hash + Eq {}

// Type for generic Exp.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Exp<B> {
    Act(u64),
    Seq(Rc<Exp<B>>, Rc<Exp<B>>),
    Ifte(B, Rc<Exp<B>>, Rc<Exp<B>>),
    Test(B),
    While(B, Rc<Exp<B>>),
}

// Trait for generic Gkat manager.
pub trait Gkat<B: Clone + Hash + Eq> {
    // Methods for BExp.
    fn mk_zero(&mut self) -> B;
    fn mk_one(&mut self) -> B;
    fn mk_var(&mut self, s: String) -> B;
    fn mk_and(&mut self, b1: &B, b2: &B) -> B;
    fn mk_or(&mut self, b1: &B, b2: &B) -> B;
    fn mk_not(&mut self, b: &B) -> B;
    fn is_false(&mut self, b: &B) -> bool;
    fn is_equiv(&mut self, b1: &B, b2: &B) -> bool;

    // Create a new BExp from parsing.
    fn from_bexp(&mut self, raw: parsing::BExp) -> B {
        use parsing::BExp::*;
        match raw {
            Zero => self.mk_zero(),
            One => self.mk_one(),
            PBool(s) => self.mk_var(s),
            Or(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                self.mk_or(&b1, &b2)
            }
            And(b1, b2) => {
                let b1 = self.from_bexp(*b1);
                let b2 = self.from_bexp(*b2);
                self.mk_and(&b1, &b2)
            }
            Not(b) => {
                let b = self.from_bexp(*b);
                self.mk_not(&b)
            }
        }
    }

    // Create a new Exp from parsing.
    fn from_exp(&mut self, raw: parsing::Exp) -> Exp<B> {
        use parsing::Exp::*;
        match raw {
            Act(s) => {
                // Convert string to u64 hash
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                let a = hasher.finish();
                Exp::Act(a)
            },
            Seq(e1, e2) => {
                let e1 = self.from_exp(*e1);
                let e2 = self.from_exp(*e2);
                Exp::Seq(Rc::new(e1), Rc::new(e2))
            }
            Ifte(b, e1, e2) => {
                let b = self.from_bexp(b);
                let e1 = self.from_exp(*e1);
                let e2 = self.from_exp(*e2);
                Exp::Ifte(b, Rc::new(e1), Rc::new(e2))
            }
            Test(b) => {
                let b = self.from_bexp(b);
                Exp::Test(b)
            }
            While(b, e) => {
                let b = self.from_bexp(b);
                let e = self.from_exp(*e);
                Exp::While(b, Rc::new(e))
            }
        }
    }
}
