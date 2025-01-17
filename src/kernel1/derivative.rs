use super::*;

impl Solver {
    pub fn epsilon(&mut self, gkat: &mut Gkat, m: &Exp) -> BExp {
        if let Some(eps) = self.get_eps(m) {
            return eps.clone();
        }
        use Exp_::*;
        let eps = match m.get() {
            Act(_) => gkat.zero(),
            Seq(p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                b1.and(&b2)
            }
            Ifte(b, p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                let b_b1 = b.and(&b1);
                let nb = b.not();
                let nb_b2 = nb.and(&b2);
                b_b1.or(&nb_b2)
            }
            Test(b) => b.clone(),
            While(b, _) => b.not(),
        };
        self.set_eps(m.clone(), eps.clone());
        return eps;
    }

    pub fn derivative(&mut self, gkat: &mut Gkat, exp: &Exp) -> Deriv {
        if let Some(deriv) = self.get_drv(exp) {
            return deriv.clone();
        }
        use Exp_::*;
        let deriv = match exp.get() {
            Test(_) => vec![],
            Act(n) => {
                let one_exp = gkat.one();
                let e = gkat.mk_test(one_exp.clone());
                vec![(one_exp, e, *n)]
            }
            Ifte(b, p1, p2) => {
                let nb = b.not();
                let dexp1 = self.derivative(gkat, p1);
                let dexp2 = self.derivative(gkat, p2);
                let mut dexp: Vec<_> = GuardIterator::new(b, dexp1.iter()).collect();
                let dexp_ext = GuardIterator::new(&nb, dexp2.iter());
                dexp.extend(dexp_ext);
                dexp
            }
            Seq(p1, p2) => {
                let eps = self.epsilon(gkat, p1);
                let mut dexp = self.derivative(gkat, p1);
                let dexp2 = self.derivative(gkat, p2);
                for (_, e, _) in dexp.iter_mut() {
                    let seq_exp = gkat.mk_seq(e.clone(), p2.clone());
                    *e = seq_exp
                }
                let dexp_ext = GuardIterator::new(&eps, dexp2.iter());
                dexp.extend(dexp_ext);
                dexp
            }
            While(be, p) => {
                let dexp = self.derivative(gkat, p);
                dexp.into_iter()
                    .filter_map(|(b, e, a)| {
                        let guard = b.and(be);
                        if guard.is_false() {
                            None
                        } else {
                            let seq_exp = gkat.mk_seq(e.clone(), exp.clone());
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
