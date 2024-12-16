use super::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    pub fn epsilon(&mut self, m: &Exp<Ptr>) -> Ptr {
        use Exp_::*;
        match m.get() {
            Act(_) => self.mk_zero(),
            Seq(p1, p2) => {
                let b1 = self.epsilon(p1);
                let b2 = self.epsilon(p2);
                self.mk_and(b1, b2)
            }
            If(b, p1, p2) => {
                let b1 = self.epsilon(p1);
                let b2 = self.epsilon(p2);
                let b_b1 = self.mk_and(b.clone(), b1);
                let nb = self.mk_not(b.clone());
                let nb_b2 = self.mk_and(nb, b2);
                self.mk_or(b_b1, nb_b2)
            }
            Test(b) => b.clone(),
            While(b, _) => self.mk_not(b.clone()),
        }
    }

    fn combine_bexp_with(&mut self, be: Ptr, m: &mut Vec<(Ptr, Exp<Ptr>, Action)>) {
        for elem in m.iter_mut() {
            let a = self.mk_and(be.clone(), elem.0);
            (*elem).0 = a;
        }
    }

    fn combine_exp_with(&mut self, exp: &Exp<Ptr>, m: &mut Vec<(Ptr, Exp<Ptr>, Action)>) {
        for elem in m.iter_mut() {
            let seq_exp = self.mk_seq(elem.1.clone(), exp.clone());
            (*elem).1 = seq_exp;
        }
    }

    fn while_helper(&mut self, be: &Ptr, exp: &Exp<Ptr>, m: &mut Vec<(Ptr, Exp<Ptr>, Action)>) {
        let while_exp = self.mk_while(be.clone(), exp.clone());
        for elem in m.iter_mut() {
            let seq_exp = self.mk_seq(elem.1.clone(), while_exp.clone());
            let b = self.mk_and(elem.0, be.clone());
            (*elem).0 = b;
            (*elem).1 = seq_exp;
        }
    }

    pub fn derivative(&mut self, exp: &Exp<Ptr>) -> Vec<(Ptr, Exp<Ptr>, Action)> {
        if let Some(deriv) = self.deriv_cache.get(exp) {
            return deriv.clone();
        }
        use Exp_::*;
        let deriv = match exp.get() {
            Test(_) => vec![],
            Act(n) => {
                let one_exp = self.mk_one();
                let e = self.mk_test(one_exp.clone());
                vec![(one_exp, e, n.clone())]
            }
            If(be, p1, p2) => {
                let mut dexp1 = self.derivative(p1);
                let mut dexp2 = self.derivative(p2);
                let not_be = self.mk_not(be.clone());
                self.combine_bexp_with(be.clone(), &mut dexp1);
                self.combine_bexp_with(not_be, &mut dexp2);
                dexp1.append(&mut dexp2);
                dexp1
            }
            Seq(p1, p2) => {
                let eps = self.epsilon(p1);
                let mut dexp1 = self.derivative(p1);
                let mut dexp2 = self.derivative(p2);
                self.combine_exp_with(p2, &mut dexp1);
                self.combine_bexp_with(eps, &mut dexp2);
                dexp1.append(&mut dexp2);
                dexp1
            }
            While(be, p) => {
                let mut dexp = self.derivative(p);
                self.while_helper(be, p, &mut dexp);
                dexp
            }
        };
        self.deriv_cache.put(exp.clone(), deriv.clone());
        return deriv;
    }
}
