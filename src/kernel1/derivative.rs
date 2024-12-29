use super::*;

impl<A, M, Builder> Solver<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn epsilon(&mut self, gkat: &mut Gkat<A, M, Builder>, m: &Exp<BExp<A, M>>) -> BExp<A, M> {
        if let Some(eps) = self.eps_cache.get(m) {
            return *eps;
        }
        use Exp_::*;
        let eps = match m.get() {
            Act(_) => gkat.mk_zero(),
            Seq(p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                gkat.mk_and(b1, b2)
            }
            Ifte(b, p1, p2) => {
                let b1 = self.epsilon(gkat, p1);
                let b2 = self.epsilon(gkat, p2);
                let b_b1 = gkat.mk_and(*b, b1);
                let nb = gkat.mk_not(*b);
                let nb_b2 = gkat.mk_and(nb, b2);
                gkat.mk_or(b_b1, nb_b2)
            }
            Test(b) => *b,
            While(b, _) => gkat.mk_not(*b),
        };
        self.eps_cache.push(m.clone(), eps);
        return eps;
    }

    pub fn derivative(
        &mut self,
        gkat: &mut Gkat<A, M, Builder>,
        exp: &Exp<BExp<A, M>>,
    ) -> Vec<(BExp<A, M>, Exp<BExp<A, M>>, u64)> {
        if let Some(deriv) = self.deriv_cache.get(exp) {
            return deriv.clone();
        }
        use Exp_::*;
        let deriv = match exp.get() {
            Test(_) => vec![],
            Act(n) => {
                let one_exp = gkat.mk_one();
                let e = gkat.mk_test(one_exp);
                vec![(one_exp, e, *n)]
            }
            Ifte(b, p1, p2) => {
                let nb = gkat.mk_not(*b);
                let dexp1 = self.derivative(gkat, p1);
                let dexp2 = self.derivative(gkat, p2);
                let mut dexp: Vec<_> = GuardIterator::new(gkat, *b, dexp1.iter()).collect();
                let dexp_ext = GuardIterator::new(gkat, nb, dexp2.iter());
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
                let dexp_ext = GuardIterator::new(gkat, eps, dexp2.iter());
                dexp.extend(dexp_ext);
                dexp
            }
            While(be, p) => {
                let dexp = self.derivative(gkat, p);
                dexp.into_iter()
                    .filter_map(|(b, e, a)| {
                        let guard = gkat.mk_and(b, *be);
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
        self.deriv_cache.push(exp.clone(), deriv.clone());
        return deriv;
    }
}
