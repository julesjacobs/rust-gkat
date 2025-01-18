use super::*;

impl<B: BExp> Solver<B> {
    pub fn equiv_iter<G: Gkat<B>>(
        &mut self,
        gkat: &mut G,
        i: u64,
        j: u64,
        m: &Automaton<B>,
        n: &Automaton<B>,
    ) -> bool {
        let mut stack = vec![(i, j)];
        while let Some((i, j)) = stack.pop() {
            let mut exp1_uf = self.get_uf(i);
            let mut exp2_uf = self.get_uf(j);

            if exp1_uf.equiv(&exp2_uf) {
                continue;
            } else if self.known_dead(&i) && self.is_dead(gkat, j, n) {
                continue;
            } else if self.known_dead(&j) && self.is_dead(gkat, i, m) {
                continue;
            } else {
                let eps1 = m.eps_hat.get(&i).unwrap();
                let eps2 = n.eps_hat.get(&j).unwrap();
                let delta1 = m.delta_hat.get(&i).unwrap();
                let delta2 = n.delta_hat.get(&j).unwrap();
                let reject1 = self.reject(gkat, i, m);
                let reject2 = self.reject(gkat, j, n);

                if !(gkat.is_equiv(&eps1, &eps2)) {
                    return false;
                }
                let assert1 = delta2.iter().all(|(b0, st, _)| {
                    let b1 = gkat.mk_and(&reject1, b0);
                    gkat.is_false(&b1) || self.is_dead(gkat, *st, n)
                });
                if !assert1 {
                    return false;
                }
                let assert2 = delta1.iter().all(|(b0, st, _)| {
                    let b1 = gkat.mk_and(&reject2, b0);
                    gkat.is_false(&b1) || self.is_dead(gkat, *st, m)
                });
                if !assert2 {
                    return false;
                }
                for (be1, st1, p) in delta1 {
                    for (be2, st2, q) in delta2 {
                        let b1b2 = gkat.mk_and(be1, be2);
                        if gkat.is_false(&b1b2) {
                            continue;
                        } else if p == q {
                            exp1_uf.union(&mut exp2_uf);
                            stack.push((*st1, *st2));
                        } else {
                            let result1 = self.is_dead(gkat, *st1, m);
                            let result2 = self.is_dead(gkat, *st2, n);
                            if !(result1 && result2) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}
