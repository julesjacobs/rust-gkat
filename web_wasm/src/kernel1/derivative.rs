use super::*;
use std::rc::Rc;

impl<B: BExp> Solver<B> {
    pub fn epsilon<G: Gkat<B>>(&mut self, gkat: &mut G, m: &Exp<B>) -> B {
        if let Some(eps) = self.get_eps(m) {
            return eps.clone();
        }
        let eps = match m {
            Exp::Act(_) => gkat.mk_zero(),
            Exp::Seq(p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                gkat.mk_and(&b1, &b2)
            }
            Exp::Ifte(b, p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                let b_b1 = gkat.mk_and(b, &b1);
                let nb = gkat.mk_not(b);
                let nb_b2 = gkat.mk_and(&nb, &b2);
                gkat.mk_or(&b_b1, &nb_b2)
            }
            Exp::Test(b) => b.clone(),
            Exp::While(b, _) => gkat.mk_not(b),
        };
        self.set_eps(m.clone(), eps.clone());
        return eps;
    }

    pub fn derivative<G: Gkat<B>>(&mut self, gkat: &mut G, exp: &Exp<B>) -> Deriv<B> {
        if let Some(deriv) = self.get_drv(exp) {
            return deriv.clone();
        }
        let deriv = match exp {
            Exp::Test(_) => vec![],
            Exp::Act(n) => {
                let one_exp = gkat.mk_one();
                let e = Exp::Test(one_exp.clone());
                vec![(one_exp, e, *n)]
            }
            Exp::Ifte(b, p1, p2) => {
                let nb = gkat.mk_not(b);
                let dexp1 = self.derivative(gkat, p1);
                let dexp2 = self.derivative(gkat, p2);
                let mut dexp: Vec<_> = GuardIterator::new(gkat, b, dexp1.iter()).collect();
                let dexp_ext = GuardIterator::new(gkat, &nb, dexp2.iter());
                dexp.extend(dexp_ext);
                dexp
            }
            Exp::Seq(p1, p2) => {
                let eps = self.epsilon(gkat, p1);
                let mut dexp = self.derivative(gkat, p1);
                let dexp2 = self.derivative(gkat, p2);
                for (_, e, _) in dexp.iter_mut() {
                    let seq_exp = Exp::Seq(Rc::new(e.clone()), p2.clone());
                    *e = seq_exp
                }
                let dexp_ext = GuardIterator::new(gkat, &eps, dexp2.iter());
                dexp.extend(dexp_ext);
                dexp
            }
            Exp::While(be, p) => {
                let dexp = self.derivative(gkat, p);
                dexp.into_iter()
                    .filter_map(|(b, e, a)| {
                        let guard = gkat.mk_and(&b, be);
                        if gkat.is_false(&guard) {
                            None
                        } else {
                            let seq_exp = Exp::Seq(Rc::new(e.clone()), Rc::new(exp.clone()));
                            Some((guard, seq_exp, a))
                        }
                    })
                    .collect()
            }
        };
        self.set_drv(exp.clone(), deriv.clone());
        return deriv;
    }
}
