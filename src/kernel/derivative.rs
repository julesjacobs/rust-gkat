use super::*;
use rsdd::{builder::BottomUpBuilder, repr::BddPtr};

impl<'a, Builder: BottomUpBuilder<'a, BddPtr<'a>>> GkatManager<'a, Builder> {
    pub fn epsilon(&mut self, m: &Exp) -> BExp {
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

    fn combine_bexp_with(
        &mut self,
        be: BExp,
        m: Vec<(BExp, (Exp, Action))>,
    ) -> Vec<(BExp, (Exp, Action))> {
        m.into_iter()
            .map(|(a, b)| {
                let a = self.mk_and(be.clone(), a);
                (a, b)
            })
            .collect()
    }

    fn while_helper(
        &mut self,
        be: &BExp,
        exp: &Exp,
        m: Vec<(BExp, (Exp, Action))>,
    ) -> Vec<(BExp, (Exp, Action))> {
        m.into_iter()
            .map(|(a, (e, p))| {
                let while_exp = self.mk_while(be.clone(), exp.clone());
                let seq_exp = self.mk_seq(e, while_exp);
                let b = self.mk_and(a, be.clone());
                (b, (seq_exp, p))
            })
            .collect()
    }

    fn seq_helper_no_epsilon(
        &mut self,
        exp: &Exp,
        m: Vec<(BExp, (Exp, Action))>,
    ) -> Vec<(BExp, (Exp, Action))> {
        m.into_iter()
            .map(|(b, (e, p))| {
                let seq_exp = self.mk_seq(e, exp.clone());
                (b, (seq_exp, p))
            })
            .collect()
    }

    fn seq_helper_epsilon(
        &mut self,
        eps: &BExp,
        m: Vec<(BExp, (Exp, Action))>,
    ) -> Vec<(BExp, (Exp, Action))> {
        m.into_iter()
            .map(|(b, pair)| {
                let b = self.mk_and(b, eps.clone());
                (b, pair)
            })
            .collect()
    }

    pub fn derivative(&mut self, exp: &Exp) -> Vec<(BExp, (Exp, Action))> {
        use Exp_::*;
        match exp.get() {
            Test(_) => vec![],
            Act(n) => {
                let one_exp = self.mk_one();
                let e = self.mk_test(one_exp.clone());
                vec![(one_exp, (e, n.clone()))]
            }
            If(be, p1, p2) => {
                let dexp1 = self.derivative(p1);
                let dexp2 = self.derivative(p2);
                let not_be = self.mk_not(be.clone());
                let mut combine1 = self.combine_bexp_with(be.clone(), dexp1);
                let mut combine2 = self.combine_bexp_with(not_be, dexp2);
                combine1.append(&mut combine2);
                combine1
            }
            Seq(p1, p2) => {
                let eps = self.epsilon(p1);
                let dexp1 = self.derivative(p1);
                let dexp2 = self.derivative(p2);
                let mut seq1 = self.seq_helper_no_epsilon(p2, dexp1);
                let mut seq2 = self.seq_helper_epsilon(&eps, dexp2);
                seq1.append(&mut seq2);
                seq1
            }
            While(be, p) => {
                let dexp = self.derivative(p);
                self.while_helper(be, p, dexp)
            }
        }
    }
}
