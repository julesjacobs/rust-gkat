use super::*;
use crate::parsing;
use rsdd::{
    builder::BottomUpBuilder,
    repr::{DDNNFPtr, VarLabel},
};

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> Gkat<'a, Ptr, Builder> {
    fn mk_name(&mut self, s: String) -> VarLabel {
        match self.name_map.get(&s) {
            Some(x) => x.clone(),
            None => {
                self.name_stamp += 1;
                let x = VarLabel::new(self.name_stamp);
                self.name_map.insert(s.clone(), x.clone());
                x
            }
        }
    }

    pub fn mk_zero(&mut self) -> Ptr {
        self.bexp_builder.false_ptr()
    }

    pub fn mk_one(&mut self) -> Ptr {
        self.bexp_builder.true_ptr()
    }

    pub fn mk_pbool(&mut self, s: String) -> Ptr {
        let x = self.mk_name(s);
        self.bexp_builder.var(x, true)
    }

    pub fn mk_or(&mut self, b1: Ptr, b2: Ptr) -> Ptr {
        if b1.is_true() {
            self.mk_one()
        } else if b2.is_true() {
            self.mk_one()
        } else if b1.is_false() {
            b2
        } else if b2.is_false() {
            b1
        } else if b1 == b2 {
            b1
        } else {
            self.bexp_builder.or(b1, b2)
        }
    }

    pub fn mk_and(&mut self, b1: Ptr, b2: Ptr) -> Ptr {
        if b1.is_true() {
            b2
        } else if b2.is_true() {
            b1
        } else if b1.is_false() {
            self.mk_zero()
        } else if b2.is_false() {
            self.mk_zero()
        } else if b1 == b2 {
            b1
        } else {
            self.bexp_builder.and(b1, b2)
        }
    }

    pub fn mk_not(&mut self, b1: Ptr) -> Ptr {
        if b1.is_true() {
            self.mk_zero()
        } else if b1.is_false() {
            self.mk_one()
        } else {
            self.bexp_builder.negate(b1)
        }
    }

    pub fn from_bexp(&mut self, raw: parsing::BExp) -> Ptr {
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
}
