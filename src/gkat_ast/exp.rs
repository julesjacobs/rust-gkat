use crate::*;
use hashconsing::{HConsed, HConsign, HashConsign};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Eq)]
pub struct Action {
    name: String,
    id: u64,
}

impl Action {
    pub fn mk(s: String) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let id = hasher.finish();
        Action { name: s, id: id }
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

pub fn mk_act(factory: &mut HConsign<Exp_>, a: Action) -> Exp {
    factory.mk(Exp_::Act(a))
}

pub fn mk_skip(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>) -> Exp {
    let b = mk_one(fb);
    mk_test(fp, b)
}

pub fn mk_fail(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>) -> Exp {
    let b = mk_zero(fb);
    mk_test(fp, b)
}

pub fn mk_seq(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>, p1: Exp, p2: Exp) -> Exp {
    use Exp_::*;
    match (p1.get(), p2.get()) {
        (Test(b1), Test(b2)) => {
            let b1 = b1.clone();
            let b2 = b2.clone();
            mk_test(fp, mk_and(fb, b1, b2))
        }
        _ => {
            if p1 == mk_skip(fb, fp) {
                p2
            } else if p2 == mk_skip(fb, fp) {
                p1
            } else if p1 == mk_fail(fb, fp) {
                mk_fail(fb, fp)
            } else if p2 == mk_fail(fb, fp) {
                mk_fail(fb, fp)
            } else {
                fp.mk(Exp_::Seq(p1, p2))
            }
        }
    }
}

pub fn mk_if(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>, b: BExp, p1: Exp, p2: Exp) -> Exp {
    if b == mk_one(fb) {
        p1
    } else if b == mk_zero(fb) {
        p2
    } else if p1 == mk_fail(fb, fp) {
        let p1 = mk_test(fp, mk_not(fb, b));
        mk_seq(fb, fp, p1, p2)
    } else if p2 == mk_fail(fb, fp) {
        let p0 = mk_test(fp, b);
        mk_seq(fb, fp, p0, p1)
    } else {
        fp.mk(Exp_::Seq(p1, p2))
    }
}

pub fn mk_test(fp: &mut HConsign<Exp_>, b: BExp) -> Exp {
    fp.mk(Exp_::Test(b))
}

pub fn mk_while(fp: &mut HConsign<Exp_>, b: BExp, p: Exp) -> Exp {
    fp.mk(Exp_::While(b, p))
}
