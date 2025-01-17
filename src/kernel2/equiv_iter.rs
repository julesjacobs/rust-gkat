use super::*;

impl Solver {
    pub fn equiv_iter(&mut self, i: u64, j: u64, m: &Automaton, n: &Automaton) -> bool {
        let mut stack = Vec::new();
        stack.push((i, j));
        while let Some((i, j)) = stack.pop() {
            let mut exp1_uf = self.get_uf(i);
            let mut exp2_uf = self.get_uf(j);

            if exp1_uf.equiv(&exp2_uf) {
                continue;
            } else if self.known_dead(&i) && self.is_dead(j, n) {
                continue;
            } else if self.known_dead(&j) && self.is_dead(i, m) {
                continue;
            } else {
                let eps1 = m.eps_hat.get(&i).unwrap();
                let eps2 = n.eps_hat.get(&j).unwrap();
                let delta1 = m.delta_hat.get(&i).unwrap();
                let delta2 = n.delta_hat.get(&j).unwrap();
                let reject1 = self.reject(i, m);
                let reject2 = self.reject(j, n);

                if !(eps1 == eps2) {
                    return false;
                }
                let assert1 = delta2.iter().all(|(b0, st, _)| {
                    let b1 = reject1.and(b0);
                    b1.is_false() || self.is_dead(*st, n)
                });
                if !assert1 {
                    return false;
                }
                let assert2 = delta1.iter().all(|(b0, st, _)| {
                    let b1 = reject2.and(b0);
                    b1.is_false() || self.is_dead(*st, m)
                });
                if !assert2 {
                    return false;
                }
                for (be1, st1, p) in delta1 {
                    for (be2, st2, q) in delta2 {
                        let b1b2 = be1.and(be2);
                        if b1b2.is_false() {
                            continue;
                        } else if p == q {
                            exp1_uf.union(&mut exp2_uf);
                            stack.push((*st1, *st2));
                        } else {
                            let result1 = self.is_dead(*st1, m);
                            let result2 = self.is_dead(*st2, n);
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
