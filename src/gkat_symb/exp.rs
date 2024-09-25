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

pub fn epsilon(fb: &mut HConsign<BExp_>, m: &Exp) -> BExp {
    use Exp_::*;
    match m.get() {
        Act(_) => mk_zero(fb),
        Seq(p1, p2) => {
            let b1 = epsilon(fb, p1);
            let b2 = epsilon(fb, p2);
            mk_and(fb, b1, b2)
        }
        If(b, p1, p2) => {
            let b1 = epsilon(fb, p1);
            let b2 = epsilon(fb, p2);
            let b_b1 = mk_and(fb, b.clone(), b1);
            let nb = mk_not(fb, b.clone());
            let nb_b2 = mk_and(fb, nb, b2);
            mk_or(fb, b_b1, nb_b2)
        }
        Test(b) => b.clone(),
        While(b, _) => mk_not(fb, b.clone()),
    }
}

fn combine_bexp_with(
    fb: &mut HConsign<BExp_>,
    be: BExp,
    m: Vec<(BExp, (Exp, Action))>,
) -> Vec<(BExp, (Exp, Action))> {
    m.into_iter()
        .map(|(a, b)| {
            let a = mk_and(fb, be.clone(), a);
            (a, b)
        })
        .collect()
}

fn while_helper(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    be: &BExp,
    exp: &Exp,
    m: Vec<(BExp, (Exp, Action))>,
) -> Vec<(BExp, (Exp, Action))> {
    m.into_iter()
        .map(|(a, (e, p))| {
            let while_exp = mk_while(fp, be.clone(), exp.clone());
            let seq_exp = mk_seq(fb, fp, e, while_exp);
            let b = mk_and(fb, a, be.clone());
            (b, (seq_exp, p))
        })
        .collect()
}

fn seq_helper_no_epsilon(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    exp: &Exp,
    m: Vec<(BExp, (Exp, Action))>,
) -> Vec<(BExp, (Exp, Action))> {
    m.into_iter()
        .map(|(b, (e, p))| {
            let seq_exp = mk_seq(fb, fp, e, exp.clone());
            (b, (seq_exp, p))
        })
        .collect()
}

fn seq_helper_epsilon(
    fb: &mut HConsign<BExp_>,
    eps: &BExp,
    m: Vec<(BExp, (Exp, Action))>,
) -> Vec<(BExp, (Exp, Action))> {
    m.into_iter()
        .map(|(b, pair)| {
            let b = mk_and(fb, b, eps.clone());
            (b, pair)
        })
        .collect()
}

pub fn derivative(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    exp: &Exp,
) -> Vec<(BExp, (Exp, Action))> {
    use Exp_::*;
    match exp.get() {
        Test(_) => vec![],
        Act(n) => {
            let one_exp = mk_one(fb);
            let e = mk_test(fp, one_exp.clone());
            vec![(one_exp, (e, n.clone()))]
        }
        If(be, p1, p2) => {
            let dexp1 = derivative(fb, fp, p1);
            let dexp2 = derivative(fb, fp, p2);
            let not_be = mk_not(fb, be.clone());
            let mut combine1 = combine_bexp_with(fb, be.clone(), dexp1);
            let mut combine2 = combine_bexp_with(fb, not_be, dexp2);
            combine1.append(&mut combine2);
            combine1
        }
        Seq(p1, p2) => {
            let eps = epsilon(fb, p1);
            let dexp1 = derivative(fb, fp, p1);
            let dexp2 = derivative(fb, fp, p2);
            let mut seq1 = seq_helper_no_epsilon(fb, fp, p2, dexp1);
            let mut seq2 = seq_helper_epsilon(fb, &eps, dexp2);
            seq1.append(&mut seq2);
            seq1
        }
        While(be, p) => {
            let dexp = derivative(fb, fp, p);
            while_helper(fb, fp, be, p, dexp)
        }
    }
}
