use bexp::NameBuilder;
use exp::{mk_act, mk_if, mk_seq, mk_test, mk_while, Action};
use hashconsing::HConsign;

use crate::{gkat_ast::*, mk_and, mk_not, mk_one, mk_or, mk_pbool, mk_zero, BExp_, Exp_};

#[derive(Debug, Clone)]
pub enum BExp {
    Zero,
    One,
    PBool(String),
    Or(Box<BExp>, Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Not(Box<BExp>),
}

#[derive(Debug, Clone)]
pub enum Exp {
    Act(String),
    Seq(Box<Exp>, Box<Exp>),
    If(BExp, Box<Exp>, Box<Exp>),
    Test(BExp),
    While(BExp, Box<Exp>),
}

impl BExp {
    pub fn to_hashcons(self, nb: &mut NameBuilder, fb: &mut HConsign<BExp_>) -> bexp::BExp {
        match self {
            Self::Zero => mk_zero(fb),
            Self::One => mk_one(fb),
            Self::PBool(s) => mk_pbool(nb, fb, s),
            Self::Or(b1, b2) => {
                let b1 = b1.to_hashcons(nb, fb);
                let b2 = b2.to_hashcons(nb, fb);
                mk_or(fb, b1, b2)
            }
            Self::And(b1, b2) => {
                let b1 = b1.to_hashcons(nb, fb);
                let b2 = b2.to_hashcons(nb, fb);
                mk_and(fb, b1, b2)
            }
            Self::Not(b) => {
                let b = b.to_hashcons(nb, fb);
                mk_not(fb, b)
            }
        }
    }
}

impl Exp {
    pub fn to_hashcons(
        self,
        nb: &mut NameBuilder,
        fb: &mut HConsign<BExp_>,
        fp: &mut HConsign<Exp_>,
    ) -> exp::Exp {
        match self {
            Self::Act(s) => mk_act(fp, s),
            Self::Seq(p1, p2) => {
                let p1 = p1.to_hashcons(nb, fb, fp);
                let p2 = p2.to_hashcons(nb, fb, fp);
                mk_seq(fb, fp, p1, p2)
            }
            Self::If(b, p1, p2) => {
                let b = b.to_hashcons(nb, fb);
                let p1 = p1.to_hashcons(nb, fb, fp);
                let p2 = p2.to_hashcons(nb, fb, fp);
                mk_if(fb, fp, b, p1, p2)
            }
            Self::Test(b) => {
                let b = b.to_hashcons(nb, fb);
                mk_test(fp, b)
            }
            Self::While(b, p) => {
                let b = b.to_hashcons(nb, fb);
                let p = p.to_hashcons(nb, fb, fp);
                mk_while(fp, b, p)
            }
        }
    }
}
