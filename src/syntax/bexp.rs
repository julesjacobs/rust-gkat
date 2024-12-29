use super::*;
use crate::parsing;
use rsdd::{
    builder::BottomUpBuilder,
    repr::{DDNNFPtr, VarLabel},
};

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Gkat<'a, BExp, Builder> {
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

    pub fn mk_zero(&mut self) -> BExp {
        self.bexp_builder.false_ptr()
    }

    pub fn mk_one(&mut self) -> BExp {
        self.bexp_builder.true_ptr()
    }

    pub fn mk_pbool(&mut self, s: String) -> BExp {
        let x = self.mk_name(s);
        self.bexp_builder.var(x, true)
    }

    pub fn mk_or(&mut self, b1: BExp, b2: BExp) -> BExp {
        self.bexp_builder.or(b1, b2)
    }

    pub fn mk_and(&mut self, b1: BExp, b2: BExp) -> BExp {
        self.bexp_builder.and(b1, b2)
    }

    pub fn mk_not(&mut self, b1: BExp) -> BExp {
        self.bexp_builder.negate(b1)
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
}
