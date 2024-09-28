use super::epsilon::*;
use crate::gkat_ast::*;
use hashconsing::HConsign;

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
