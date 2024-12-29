use super::*;
use crate::parsing;

pub type BExp<A, M> = NodeIndex<A, M>;

impl<A, M, Builder> Gkat<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn mk_zero(&mut self) -> BExp<A, M> {
        NodeIndex::FALSE
    }

    pub fn mk_one(&mut self) -> BExp<A, M> {
        NodeIndex::TRUE
    }

    pub fn mk_pbool(&mut self, s: String) -> BExp<A, M> {
        if let Some(x) = self.name_map.get(&s) {
            return self.bexp_builder.single_variable(*x);
        }
        self.name_stamp += 1;
        let x = VariableIndex(self.name_stamp as u16);
        self.name_map.insert(s, x);
        self.bexp_builder.single_variable(x)
    }

    pub fn mk_or(&mut self, b1: BExp<A, M>, b2: BExp<A, M>) -> BExp<A, M> {
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

    pub fn mk_and(&mut self, b1: BExp<A, M>, b2: BExp<A, M>) -> BExp<A, M> {
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

    pub fn mk_not(&mut self, b1: BExp<A, M>) -> BExp<A, M> {
        if b1.is_true() {
            self.mk_zero()
        } else if b1.is_false() {
            self.mk_one()
        } else {
            self.bexp_builder.not(b1)
        }
    }

    pub fn from_bexp(&mut self, raw: parsing::BExp) -> BExp<A, M> {
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
