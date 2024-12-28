use super::*;
use crate::syntax::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    pub fn epsilon(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, m: &Exp<BExp>) -> BExp {
        use Exp_::*;
        match m.get() {
            Act(_) => gkat.mk_zero(),
            Seq(p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                gkat.mk_and(b1, b2)
            }
            Ifte(b, p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                let b_b1 = gkat.mk_and(b.clone(), b1);
                let nb = gkat.mk_not(b.clone());
                let nb_b2 = gkat.mk_and(nb, b2);
                gkat.mk_or(b_b1, nb_b2)
            }
            Test(b) => b.clone(),
            While(b, _) => gkat.mk_not(b.clone()),
        }
    }

    fn combine_bexp_with(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        be: BExp,
        m: &mut Vec<(BExp, Exp<BExp>, u64)>,
    ) {
        for elem in m.iter_mut() {
            let a = gkat.mk_and(be.clone(), elem.0);
            (*elem).0 = a;
        }
    }

    fn combine_exp_with(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        exp: &Exp<BExp>,
        m: &mut Vec<(BExp, Exp<BExp>, u64)>,
    ) {
        for elem in m.iter_mut() {
            let seq_exp = gkat.mk_seq(elem.1.clone(), exp.clone());
            (*elem).1 = seq_exp;
        }
    }

    fn while_helper(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        be: &BExp,
        exp: &Exp<BExp>,
        m: &mut Vec<(BExp, Exp<BExp>, u64)>,
    ) {
        let while_exp = gkat.mk_while(be.clone(), exp.clone());
        for elem in m.iter_mut() {
            let seq_exp = gkat.mk_seq(elem.1.clone(), while_exp.clone());
            let b = gkat.mk_and(elem.0, be.clone());
            (*elem).0 = b;
            (*elem).1 = seq_exp;
        }
    }

    pub fn derivative(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        exp: &Exp<BExp>,
    ) -> Vec<(BExp, Exp<BExp>, u64)> {
        if let Some(deriv) = self.deriv_cache.get(exp) {
            return deriv.clone();
        }
        use Exp_::*;
        let deriv = match exp.get() {
            Test(_) => vec![],
            Act(n) => {
                let one_exp = gkat.mk_one();
                let e = gkat.mk_test(one_exp.clone());
                vec![(one_exp, e, n.clone())]
            }
            Ifte(be, p1, p2) => {
                let mut dexp1 = self.derivative(gkat, p1);
                let mut dexp2 = self.derivative(gkat, p2);
                let not_be = gkat.mk_not(be.clone());
                self.combine_bexp_with(gkat, be.clone(), &mut dexp1);
                self.combine_bexp_with(gkat, not_be, &mut dexp2);
                dexp1.append(&mut dexp2);
                dexp1
            }
            Seq(p1, p2) => {
                let eps = self.epsilon(gkat, p1);
                let mut dexp1 = self.derivative(gkat, p1);
                let mut dexp2 = self.derivative(gkat, p2);
                self.combine_exp_with(gkat, p2, &mut dexp1);
                self.combine_bexp_with(gkat, eps, &mut dexp2);
                dexp1.append(&mut dexp2);
                dexp1
            }
            While(be, p) => {
                let mut dexp = self.derivative(gkat, p);
                self.while_helper(gkat, be, p, &mut dexp);
                dexp
            }
        };
        self.deriv_cache.put(exp.clone(), deriv.clone());
        return deriv;
    }
}
